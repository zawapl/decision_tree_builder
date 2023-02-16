use proc_macro2::{Ident, Literal, TokenStream};
use quote::{quote, ToTokens};

pub struct Decision();

impl Decision {
    pub fn to_tokens_eq<T: ToTokens>(v: T, ident: Ident) -> TokenStream {
        return quote!(t.#ident == #v);
    }

    pub fn to_tokens_ord<T: ToTokens>(v: T, ident: Ident) -> TokenStream {
        return quote!(t.#ident < #v);
    }
}
