use darling::{FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput, Ident, Type, Variant};

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
#[darling(attributes(namewise_try_from))]
struct Params {
    ident: Ident,
    data: darling::ast::Data<Variant, TryFromField>,
    #[darling(multiple, rename = "try_from_type")]
    types: Vec<Type>,
}

#[derive(Debug, Clone, FromField)]
#[darling(attributes(namewise_try_from))]
struct TryFromField {
    ident: Option<Ident>,
    #[darling(default)]
    optional: bool,
    from_name: Option<Ident>,
    mapper: Option<Type>,
}

fn derive_struct(
    destination: Ident,
    source: Type,
    fields: Vec<TryFromField>,
) -> proc_macro2::TokenStream {
    let field_mappings: Vec<proc_macro2::TokenStream> = fields
        .into_iter()
        .map(|field| {
            let field_name = field.ident.expect("Encountered an unnamed field");
            let from_option = field.optional;
            let from_name = field.from_name.unwrap_or_else(|| field_name.clone());
            let mapper = field.mapper.unwrap_or_else(|| parse_quote!(std::convert::identity));
            if from_option {
                quote! {
                    #field_name: {
                        let err_message = || ::namewise::NamewiseError::MissingField(format!("Value {}.{} is missing", stringify!(#source), stringify!(#field_name)));
                        let src_value = s.#from_name.ok_or_else(err_message)?;
                        #mapper(src_value).try_into()?
                    }
                }
            } else {
                quote! {
                    #field_name: #mapper(s.#from_name).try_into()?
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
