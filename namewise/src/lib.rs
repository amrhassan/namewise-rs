use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Field, Ident};

#[derive(FromDeriveInput)]
#[darling(attributes(namewise))]
struct Params {
    ident: Ident,
    data: darling::ast::Data<(), Field>,
    #[darling(multiple, rename = "from")]
    from_types: Vec<Ident>,
}

#[proc_macro_derive(From, attributes(namewise))]
pub fn derive_namewise_from(ts: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(ts as DeriveInput);
    let params = Params::from_derive_input(&derive_input).expect("Failed to parse inputs");

    let destination = params.ident;

    let impls: Vec<proc_macro2::TokenStream> = params
        .from_types
        .into_iter()
        .map(|source| match params.data.clone() {
            darling::ast::Data::Struct(s) => {
                derive_namewise_from_struct(destination.clone(), source, s.fields)
            }
            _ => panic!("Deriving namewise::From only works on structs"),
        })
        .collect();

    quote! {
        #(#impls) *
    }
    .into()
}

fn derive_namewise_from_struct(
    destination: Ident,
    source: Ident,
    fields: Vec<Field>,
) -> proc_macro2::TokenStream {
    let field_idents = fields
        .into_iter()
        .map(|field| field.ident.expect("Encountered a field without an ident"));

    let field_mappings: Vec<proc_macro2::TokenStream> = field_idents
        .map(|field_ident| {
            quote! {
                #field_ident: s.#field_ident.into()
            }
        })
        .collect();

    quote! {
        impl std::convert::From<#source> for #destination{
            fn from(s: #source) -> #destination {
                #destination {
                    #(#field_mappings),*
                }
            }
        }
    }
}
