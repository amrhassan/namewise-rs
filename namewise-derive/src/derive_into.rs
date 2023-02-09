use darling::{FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, Type, Variant};

pub fn derive_namewise_into(ts: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(ts as DeriveInput);
    let params = Params::from_derive_input(&derive_input).expect("Failed to parse inputs");

    let source = params.ident;

    let impls: Vec<proc_macro2::TokenStream> = params
        .types
        .into_iter()
        .map(|destination| match params.data.clone() {
            darling::ast::Data::Struct(fields) => {
                derive_struct(source.clone(), destination, fields.fields)
            }
            darling::ast::Data::Enum(variants) => {
                derive_enum(source.clone(), destination, variants)
            }
        })
        .collect();

    quote! {
        #(#impls) *
    }
    .into()
}

#[derive(FromDeriveInput)]
#[darling(attributes(namewise_into))]
struct Params {
    ident: Ident,
    data: darling::ast::Data<Variant, NIntoField>,
    #[darling(multiple, rename = "into_type")]
    types: Vec<Type>,
}

#[derive(Debug, Clone, FromField)]
#[darling(attributes(namewise_into))]
struct NIntoField {
    /// Source field name
    ident: Option<Ident>,
    /// Destination field name, otherwise same as source
    into_name: Option<Ident>,
    /// Mapper function over the source field
    mapper: Option<Type>,
    /// Treat source field as iterable that has `IntoIterator` impl with a corresponding `FromIterator` on the target
    collect: Option<bool>,
}

fn derive_struct(
    source: Ident,
    destination: Type,
    fields: Vec<NIntoField>,
) -> proc_macro2::TokenStream {
    let field_mappings: Vec<proc_macro2::TokenStream> = fields
        .into_iter()
        .map(|field| {
            let field_name = field.ident.expect("Encountered an unnamed field");
            let into_name = field.into_name.unwrap_or_else(|| field_name.clone());
            match (field.mapper, field.collect.unwrap_or_default()) {
                (None, false) => {
                    quote! {
                        #into_name: self.#field_name.into()
                    }
                }
                (Some(mapper), false) => {
                    quote! {
                        #into_name: #mapper(self.#field_name).into()
                    }
                }
                (None, true) => {
                    quote! {
                        #into_name: self.#field_name.into_iter().map(|v| v.into()).collect()
                    }
                }
                (Some(mapper), true) => {
                    quote! {
                        #into_name: self.#field_name.into_iter().map(|v| #mapper(v)).collect()
                    }
                }
            }
        })
        .collect();

    quote! {
        #[automatically_derived]
        impl ::std::convert::Into<#destination> for #source {
            fn into(self) -> #destination {
                #destination {
                    #(#field_mappings),*
                }
            }
        }
    }
}

fn derive_enum(
    source: Ident,
    destination: Type,
    variants: Vec<Variant>,
) -> proc_macro2::TokenStream {
    let variant_names = variants.into_iter().map(|variant| variant.ident);

    let variant_mappings: Vec<proc_macro2::TokenStream> = variant_names
        .map(|name| {
            quote! {
                #source::#name => #destination::#name
            }
        })
        .collect();

    quote! {
        impl ::std::convert::Into<#destination> for #source {
            fn into(self) -> #destination {
                match self {
                    #(#variant_mappings),*
                }
            }
        }
    }
}
