use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(From, attributes(namewise))]
pub fn derive_namewise_from(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        ident: destination,
        data,
        generics,
        attrs,
        ..
    } = parse_macro_input!(input);

    let struct_data = match data {
        syn::Data::Struct(s) => s,
        _ => panic!("Deriving namewise::From only works on structs"),
    };

    let named_fields = match struct_data.fields {
        syn::Fields::Named(f) => f,
        _ => panic!("Deriving namewise::From only works on structs with named fields"),
    };

    let field_idents = named_fields
        .named
        .into_iter()
        .map(|field| field.ident.expect("Encountered a field without an ident"));

    let from_ident: Ident = attrs
        .into_iter()
        .flat_map(|attr| attr.parse_meta().ok())
        .flat_map(|meta| match meta {
            syn::Meta::List(syn::MetaList { nested, path, .. }) if path.is_ident("namewise") => {
                nested.into_iter()
            }
            _ => panic!("Unexpected format of #[namewise(from(X))]"),
        })
        .map(|nested_meta| match nested_meta {
            syn::NestedMeta::Meta(meta) => meta,
            _ => panic!("Unexpected format of #[namewise(from(X))]"),
        })
        .flat_map(|meta| match meta {
            syn::Meta::List(syn::MetaList { nested, path, .. }) if path.is_ident("from") => {
                nested.into_iter()
            }
            _ => panic!("Unexpected format of #[namewise(from(X))]"),
        })
        .map(|nested_meta| match nested_meta {
            syn::NestedMeta::Meta(meta) => meta,
            _ => panic!("Unexpected format of #[namewise(from(X))]"),
        })
        .map(|meta| match meta {
            syn::Meta::Path(path) => path.get_ident().cloned().unwrap(),
            _ => panic!("Unexpected format of #[namewise(from(X))]"),
        })
        .next()
        .expect("Missing `#[namewise(from(X))]` attribute");

    let source = from_ident;

    let field_mappings: Vec<TokenStream> = field_idents
        .map(|ident| {
            quote! {
                #ident: s.#ident.into()
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
