use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Field, Ident, Type, Variant};

pub fn derive_namewise_from(ts: TokenStream) -> TokenStream {
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
#[darling(attributes(namewise_from))]
struct Params {
    ident: Ident,
    data: darling::ast::Data<Variant, Field>,
    #[darling(multiple, rename = "from_type")]
    types: Vec<Type>,
}

fn derive_struct(destination: Ident, source: Type, fields: Vec<Field>) -> proc_macro2::TokenStream {
    let field_names = fields
        .into_iter()
        .map(|field| field.ident.expect("Encountered an unnamed field"));

    let field_mappings: Vec<proc_macro2::TokenStream> = field_names
        .map(|field_name| {
            quote! {
                #field_name: s.#field_name.into()
            }
        })
        .collect();

    quote! {
        #[automatically_derived]
        impl ::std::convert::From<#source> for #destination{
            fn from(s: #source) -> #destination {
                #destination {
                    #(#field_mappings),*
                }
            }
        }
    }
}

fn derive_enum(
    destination: Ident,
    source: Type,
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