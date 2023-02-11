use darling::{FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, Type, Variant};

pub fn derive_namewise_from(ts: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(ts as DeriveInput);
    let params = Params::from_derive_input(&derive_input).expect("Failed to parse inputs");

    let destination = params.ident;

    let impls: Vec<proc_macro2::TokenStream> = params
        .types
        .into_iter()
        .map(|source| match params.data.clone() {
            darling::ast::Data::Struct(fields) => {
                derive_struct(source, destination.clone(), fields.fields)
            }
            darling::ast::Data::Enum(variants) => {
                derive_enum(source, destination.clone(), variants)
            }
        })
        .collect();

    quote! {
        #(#impls) *
    }
    .into()
}

#[derive(FromDeriveInput)]
#[darling(
    attributes(namewise_from),
    supports(struct_named, enum_named, enum_unit)
)]
struct Params {
    ident: Ident,
    data: darling::ast::Data<Variant, NamewiseFromField>,
    #[darling(multiple, rename = "from_type")]
    types: Vec<Type>,
}

#[derive(Debug, Clone, FromField)]
#[darling(attributes(namewise_from))]
struct NamewiseFromField {
    /// Destination field name
    ident: Option<Ident>,
    /// Source field name, otherwise same as destination
    from_name: Option<Ident>,
    /// Mapper function over the source field
    mapper: Option<Type>,
    /// Treat source field as iterable that has `IntoIterator` impl with a corresponding `FromIterator` on the target
    collect: Option<bool>,
}

fn derive_struct(
    source: Type,
    destination: Ident,
    fields: Vec<NamewiseFromField>,
) -> proc_macro2::TokenStream {
    let field_mappings: Vec<proc_macro2::TokenStream> = fields
        .into_iter()
        .map(|field| {
            let field_name = field.ident.expect("Encountered an unnamed field");
            let from_name = field.from_name.unwrap_or_else(|| field_name.clone());
            match (field.mapper, field.collect.unwrap_or_default()) {
                (None, false) => {
                    quote! {
                        #field_name: s.#from_name.into()
                    }
                }
                (Some(mapper), false) => {
                    quote! {
                        #field_name: #mapper(s.#from_name).into()
                    }
                }
                (None, true) => {
                    quote! {
                        #field_name: s.#from_name.into_iter().map(|v| v.into()).collect()
                    }
                }
                (Some(mapper), true) => {
                    quote! {
                        #field_name: s.#from_name.into_iter().map(|v| #mapper(v)).collect()
                    }
                }
            }
        })
        .collect();

    quote! {
        #[automatically_derived]
        impl ::std::convert::From<#source> for #destination{
            fn from(s: #source) -> Self {
                #destination {
                    #(#field_mappings),*
                }
            }
        }
    }
}

fn derive_enum(
    source: Type,
    destination: Ident,
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
        impl ::std::convert::From<#source> for #destination{
            fn from(s: #source) -> #destination {
                match s {
                    #(#variant_mappings),*
                }
            }
        }
    }
}
