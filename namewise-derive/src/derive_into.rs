use darling::{FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput, Ident, Type, Variant};

pub fn derive_namewise_into(ts: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(ts as DeriveInput);
    let params = Params::from_derive_input(&derive_input).expect("Failed to parse inputs");

    let destination = params.ident;

    let impls: Vec<proc_macro2::TokenStream> = params
        .types
        .into_iter()
        .map(|source| match params.data.clone() {
            darling::ast::Data::Struct(fields) => {
                derive_struct(destination.clone(), source, fields.fields)
            }
            darling::ast::Data::Enum(variants) => {
                derive_enum(destination.clone(), source, variants)
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
    ident: Option<Ident>,
    into_name: Option<Ident>,
    mapper: Option<Type>,
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
            let mapper = field
                .mapper
                .unwrap_or_else(|| parse_quote!(std::convert::identity));
            quote! {
                #into_name: #mapper(self.#field_name).into()
            }
        })
        .collect();

    quote! {
        #[automatically_derived]
        impl ::std::convert::Into<#destination> for #source{
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
