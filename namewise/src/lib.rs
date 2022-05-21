mod derive_from;
mod derive_into;

use proc_macro::TokenStream;

#[proc_macro_derive(From, attributes(namewise))]
pub fn derive_namewise_from(ts: TokenStream) -> TokenStream {
    derive_from::derive_namewise_from(ts)
}

#[proc_macro_derive(Into, attributes(namewise))]
pub fn derive_namewise_into(ts: TokenStream) -> TokenStream {
    derive_into::derive_namewise_into(ts)
}
