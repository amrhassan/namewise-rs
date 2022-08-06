mod derive_from;
mod derive_into;
mod derive_try_from;

use proc_macro::TokenStream;

#[proc_macro_derive(From, attributes(namewise_from))]
pub fn derive_namewise_from(ts: TokenStream) -> TokenStream {
    derive_from::derive_namewise_from(ts)
}

#[proc_macro_derive(TryFrom, attributes(namewise_try_from, namewise_try_from_option))]
pub fn derive_namewise_try_from(ts: TokenStream) -> TokenStream {
    derive_try_from::derive_namewise_try_from(ts)
}

#[proc_macro_derive(Into, attributes(namewise_into))]
pub fn derive_namewise_into(ts: TokenStream) -> TokenStream {
    derive_into::derive_namewise_into(ts)
}