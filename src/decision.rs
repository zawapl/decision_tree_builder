use proc_macro2::{Ident, Literal, TokenStream};
use quote::quote;

pub enum Decision {
    BooleanDecision(Ident, bool),
    UsizeDecision(Ident, usize),
}

impl Decision {
    pub(crate) fn get_condition_tokens(&self) -> TokenStream {
        return match self {
            Decision::BooleanDecision(i, v) => {
                quote!(t.#i == #v)
            }
            Decision::UsizeDecision(i, v) => {
                let literal = Literal::usize_unsuffixed(*v);
                quote!(t.#i < #literal)
            }
        };
    }
}
