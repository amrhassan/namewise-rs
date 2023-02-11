mod derive_from;
mod derive_into;
mod derive_try_from;

use proc_macro::TokenStream;

/// Derive [From] between two structs/enums with similarly-named fields
#[proc_macro_derive(From, attributes(namewise_from))]
pub fn derive_namewise_from(ts: TokenStream) -> TokenStream {
    derive_from::derive_namewise_from(ts)
}

/// Derive [TryFrom] between two structs/enums with similarly-named fields
#[proc_macro_derive(TryFrom, attributes(namewise_try_from))]
pub fn derive_namewise_try_from(ts: TokenStream) -> TokenStream {
    derive_try_from::derive_namewise_try_from(ts)
}

/// Derive [Into] between two structs/enums with similarly-named fields
#[proc_macro_derive(Into, attributes(namewise_into))]
pub fn derive_namewise_into(ts: TokenStream) -> TokenStream {
    derive_into::derive_namewise_into(ts)
}
