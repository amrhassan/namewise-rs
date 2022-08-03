use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput, Field, Ident, Type, Variant};

pub fn derive_namewise_try_from(ts: TokenStream) -> TokenStream {
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
            darling::ast::Data::Enum(_) => {
                panic!("namewise::TryFrom cannot be derived for enums")
            }
        })
        .collect();

    quote! {
        #(#impls) *
    }
    .into()
}

#[derive(FromDeriveInput)]
#[darling(attributes(namewise_try_from, namewise_try_from_option))]
struct Params {
    ident: Ident,
    data: darling::ast::Data<Variant, Field>,
    #[darling(multiple, rename = "try_from_type")]
    types: Vec<Type>,
}

fn derive_struct(destination: Ident, source: Type, fields: Vec<Field>) -> proc_macro2::TokenStream {
    let field_mappings: Vec<proc_macro2::TokenStream> = fields
        .into_iter()
        .map(|field| {
            let field_name = field.ident.expect("Encountered an unnamed field");
            let from_option = field.attrs.iter().any(|attr| {
                attr.path
                    .segments
                    .iter()
                    .any(|segment| segment.ident == format_ident!("namewise_try_from_option"))
            });
            if from_option {
                quote! {
                    #field_name: s.#field_name
                      .ok_or_else(|| NamewiseError::MissingField(
                          format!("Value {}.{} is missing", stringify!(#source), stringify!(#field_name))))?
                      .try_into()?
                }
            } else {
                quote! {
                    #field_name: s.#field_name.into()
                }
            }
        })
        .collect();

    quote! {
        #[automatically_derived]
        impl ::std::convert::TryFrom<#source> for #destination {
            type Error = ::namewise::NamewiseError;
            fn try_from(s: #source) -> ::std::result::Result<#destination, Self::Error> {
                Ok(#destination {
                    #(#field_mappings),*
                })
            }
        }
    }
}
