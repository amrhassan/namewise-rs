use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(From, attributes(fieldwise))]
pub fn derive_fieldwise_from(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        ident: destination,
        data,
        generics,
        ..
    } = parse_macro_input!(input);

    let struct_data = match data {
        syn::Data::Struct(s) => s,
        _ => panic!("Deriving fieldwise::From only works on structs"),
    };

    let named_fields = match struct_data.fields {
        syn::Fields::Named(f) => f,
        _ => panic!("Deriving fieldwise::From only works on structs with named fields"),
    };

    let field_idents = named_fields
        .named
        .into_iter()
        .map(|field| field.ident.expect("Encountered a field without an ident"));

    let source = format_ident!("Source");

    let field_mappings: Vec<TokenStream> = field_idents
        .map(|ident| {
            quote! {
                #ident: s.#ident.into(),
            }
        })
        .collect();

    quote! {
        impl #generics std::convert::From<#source> for #destination{
            fn from(s: #source) -> #destination {
                #destination {
                    #(#field_mappings),*
                }
            }
        }
    }
    .into()
}
