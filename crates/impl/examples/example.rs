use std::hash::Hash;

use decision_tree_builder_impl::{BranchBuilder, Decision, DecisionEval, TreeBuilder};
use proc_macro2::{Ident, Punct, Spacing, Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::__private::TokenStream2;

struct TestStructData {
    a: Inner,
    b: Inner,
}

struct Inner {
    a: bool,
    b: f32,
}

impl BranchBuilder for TestStructData {
    type Decision = __TestStructDataDecision;

    #[allow(unused_assignments)]
    fn find_best_decision<R: Copy + Eq + Hash, F, D>(entropy: f64, data: &mut [(D, R)], extract: F) -> Self::Decision
    where F: Fn(&D) -> &Self {
        let decisions = [
            __TestStructDataDecision::A(BranchBuilder::find_best_decision(entropy, data, |d| &extract(d).a)),
            __TestStructDataDecision::B(BranchBuilder::find_best_decision(entropy, data, |d| &extract(d).b)),
        ];

        return decisions.into_iter().max_by(|a, b| a.to_decision_eval().cmp(b.to_decision_eval())).unwrap();
    }

    fn split_data<F, D, R>(data: &mut [(D, R)], extract: F, decision: &Self::Decision) -> usize
    where F: Fn(&D) -> &Self {
        return match decision {
            __TestStructDataDecision::A(a) => BranchBuilder::split_data(data, |d| &extract(d).a, a),
            __TestStructDataDecision::B(b) => BranchBuilder::split_data(data, |d| &extract(d).b, b),
        };
    }
}

enum __TestStructDataDecision {
    A(<Inner as BranchBuilder>::Decision),
    B(<Inner as BranchBuilder>::Decision),
}

impl Decision for __TestStructDataDecision {
    fn to_decision_eval(&self) -> &DecisionEval {
        return match self {
            __TestStructDataDecision::A(a) => a.to_decision_eval(),
            __TestStructDataDecision::B(b) => b.to_decision_eval(),
        };
    }

    fn to_condition(&self, var: TokenStream) -> TokenStream {
        return match self {
            __TestStructDataDecision::A(a) => a.to_condition(quote!(#var.a)),
            __TestStructDataDecision::B(b) => b.to_condition(quote!(#var.b)),
        };
    }
}

impl BranchBuilder for Inner {
    type Decision = __InnerDecision;

    #[allow(unused_assignments)]
    fn find_best_decision<R: Copy + Eq + Hash, F, D>(entropy: f64, data: &mut [(D, R)], extract: F) -> Self::Decision
    where F: Fn(&D) -> &Self {
        let decisions = [
            __InnerDecision::A(BranchBuilder::find_best_decision(entropy, data, |d| &extract(d).a)),
            __InnerDecision::B(BranchBuilder::find_best_decision(entropy, data, |d| &extract(d).b)),
        ];

        return decisions.into_iter().max_by(|a, b| a.to_decision_eval().cmp(b.to_decision_eval())).unwrap();
    }

    fn split_data<F, D, R>(data: &mut [(D, R)], extract: F, decision: &Self::Decision) -> usize
    where F: Fn(&D) -> &Self {
        return match decision {
            __InnerDecision::A(a) => BranchBuilder::split_data(data, |d| &extract(d).a, a),
            __InnerDecision::B(b) => BranchBuilder::split_data(data, |d| &extract(d).b, b),
        };
    }
}

enum __InnerDecision {
    A(<bool as BranchBuilder>::Decision),
    B(<f32 as BranchBuilder>::Decision),
}

impl Decision for __InnerDecision {
    fn to_decision_eval(&self) -> &DecisionEval {
        return match self {
            __InnerDecision::A(a) => a.to_decision_eval(),
            __InnerDecision::B(b) => b.to_decision_eval(),
        };
    }

    fn to_condition(&self, var: TokenStream) -> TokenStream {
        let mut result = TokenStream::new();
        result.append_all(var);
        result.append(Punct::new('.', Spacing::Alone));
        return match self {
            __InnerDecision::A(inner) => {
                result.append(Ident::new("a", Span::call_site()));
                inner.to_condition(result)
            }
            __InnerDecision::B(inner) => {
                result.append(Ident::new("b", Span::call_site()));
                inner.to_condition(result)
            }
        };
        return result.into();

        // match self {
        //     __InnerDecision::A(a) => a.to_condition(quote!(#var.a)),
        //     __InnerDecision::B(b) => b.to_condition(quote!(#var.b)),
        // };
    }
}

fn main() {
    let mut data = [
        (TestStructData { a: Inner { a: true, b: 1.0 }, b: Inner { a: true, b: 2.0 } }, "true"),
        (TestStructData { a: Inner { a: true, b: 1.0 }, b: Inner { a: true, b: 1.0 } }, "false"),
    ];
    let decision = TreeBuilder::default().build(&mut data).unwrap();
    let pretty = prettyplease::unparse(&syn::parse_file(decision.to_string().as_str()).unwrap());
    println!("{pretty}");
}
