use proc_macro2::{Literal, TokenStream};
use quote::{quote, ToTokens};

use crate::decision_eval::DecisionEval;
use crate::ToFormattedTokens;

pub trait Decision {
    fn to_decision_eval(&self) -> &DecisionEval;
    fn to_condition(&self, var: TokenStream) -> TokenStream;
}

pub struct BoolDecision {
    pub(crate) decision_eval: DecisionEval,
}

impl Decision for BoolDecision {
    fn to_decision_eval(&self) -> &DecisionEval {
        return &self.decision_eval;
    }

    fn to_condition(&self, var: TokenStream) -> TokenStream {
        return var.to_token_stream();
    }
}

pub struct EqDecision<T> {
    pub(crate) decision_eval: DecisionEval,
    pub(crate) val: T,
}

impl<T: ToTokens> Decision for EqDecision<T> {
    fn to_decision_eval(&self) -> &DecisionEval {
        return &self.decision_eval;
    }

    fn to_condition(&self, var: TokenStream) -> TokenStream {
        let val = &self.val;
        return quote!(#var == #val);
    }
}

pub struct OrdDecision<T> {
    pub(crate) decision_eval: DecisionEval,
    pub(crate) threshold: T,
}

impl<T: ToFormattedTokens> Decision for OrdDecision<T> {
    fn to_decision_eval(&self) -> &DecisionEval {
        return &self.decision_eval;
    }

    fn to_condition(&self, var: TokenStream) -> TokenStream {
        let repr = self.threshold.to_formatted_tokens();
        return quote!(#var < #repr);
    }
}

pub enum Tuple2Decision<A, B> {
    A(A),
    B(B),
}

impl<A, B> Decision for Tuple2Decision<A, B>
where
    A: Decision,
    B: Decision,
{
    fn to_decision_eval(&self) -> &DecisionEval {
        return match self {
            Tuple2Decision::A(a) => a.to_decision_eval(),
            Tuple2Decision::B(b) => b.to_decision_eval(),
        };
    }

    fn to_condition(&self, var: TokenStream) -> TokenStream {
        return match self {
            Tuple2Decision::A(a) => a.to_condition(quote!(#var.0)),
            Tuple2Decision::B(b) => b.to_condition(quote!(#var.1)),
        };
    }
}

pub struct ArrayDecision<T> {
    pub(crate) index: usize,
    pub(crate) inner_decision: T,
}

impl<T: Decision> Decision for ArrayDecision<T> {
    fn to_decision_eval(&self) -> &DecisionEval {
        return self.inner_decision.to_decision_eval();
    }

    fn to_condition(&self, var: TokenStream) -> TokenStream {
        let index = Literal::usize_unsuffixed(self.index);
        return self.inner_decision.to_condition(quote!(#var [ #index ]));
    }
}
