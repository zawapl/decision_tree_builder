use proc_macro2::{Literal, TokenStream};
use quote::{quote, ToTokens};

pub trait Decision {
    fn gain_ratio(&self) -> f64;
    fn max_branch_width(&self) -> usize;
    fn to_condition(&self, var: TokenStream) -> TokenStream;
}

#[derive(Debug)]
pub struct BoolDecision {
    pub(crate) gain_ratio: f64,
    pub(crate) max_branch_width: usize,
}

impl Decision for BoolDecision {
    fn gain_ratio(&self) -> f64 {
        return self.gain_ratio;
    }

    fn max_branch_width(&self) -> usize {
        return self.max_branch_width;
    }

    fn to_condition(&self, var: TokenStream) -> TokenStream {
        return var.to_token_stream();
    }
}

#[derive(Debug)]
pub struct EqDecision<'a, T> {
    pub(crate) gain_ratio: f64,
    pub(crate) max_branch_width: usize,
    pub(crate) val: &'a T,
}

impl<'a, T: ToTokens> Decision for EqDecision<'a, T> {
    fn gain_ratio(&self) -> f64 {
        return self.gain_ratio;
    }

    fn max_branch_width(&self) -> usize {
        return self.max_branch_width;
    }

    fn to_condition(&self, var: TokenStream) -> TokenStream {
        let val = &self.val;
        return quote!(#var == #val);
    }
}


#[derive(Debug)]
pub struct OrdDecision<T> {
    pub(crate) gain_ratio: f64,
    pub(crate) max_branch_width: usize,
    pub(crate) threshold: T,
}

impl<T: ToTokens> Decision for OrdDecision<T> {
    fn gain_ratio(&self) -> f64 {
        return self.gain_ratio;
    }

    fn max_branch_width(&self) -> usize {
        return self.max_branch_width;
    }

    fn to_condition(&self, var: TokenStream) -> TokenStream {
        let threshold_literal = &self.threshold;
        return quote!(*#var < #threshold_literal);
    }
}


#[derive(Debug)]
pub enum Tuple2Decision<A, B> {
    A(A),
    B(B),
}

impl<A, B> Decision for Tuple2Decision<A, B>
where
    A: Decision,
    B: Decision,
{
    fn gain_ratio(&self) -> f64 {
        return match self {
            Tuple2Decision::A(a) => a.gain_ratio(),
            Tuple2Decision::B(b) => b.gain_ratio(),
        };
    }

    fn max_branch_width(&self) -> usize {
        return match self {
            Tuple2Decision::A(a) => a.max_branch_width(),
            Tuple2Decision::B(b) => b.max_branch_width(),
        };
    }

    fn to_condition(&self, var: TokenStream) -> TokenStream {
        match self {
            Tuple2Decision::A(a) => {
                return a.to_condition(quote!(#var.0));
            }
            Tuple2Decision::B(b) => {
                return b.to_condition(quote!(#var.1));
            }
        };
    }
}

#[derive(Debug)]
pub struct ArrayDecision<T> {
    pub(crate) index: usize,
    pub(crate) inner_decision: T,
}

impl<T: Decision> Decision for ArrayDecision<T> {
    fn gain_ratio(&self) -> f64 {
        return self.inner_decision.gain_ratio();
    }

    fn max_branch_width(&self) -> usize {
        return self.inner_decision.max_branch_width();
    }

    fn to_condition(&self, var: TokenStream) -> TokenStream {
        let index = Literal::usize_unsuffixed(self.index);
        return self.inner_decision.to_condition(quote!(#var [ #index ]));
    }
}
