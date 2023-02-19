use std::hash::Hash;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::playground::{BranchBuilder, Decision};

struct TestStructData {
    a: Inner,
    b: Inner,
}

struct Inner {
    a: bool,
    b: bool,
}

impl BranchBuilder for TestStructData {
    type Decision = __TestStructDataDecision;

    fn find_best_decision<R: ToTokens + Copy + Eq + Hash, F, D>(entropy: f64, data: &mut [(D, R)], extract: F) -> Self::Decision
    where F: Fn(&D) -> &Self {
        let other = BranchBuilder::find_best_decision(entropy, data, |d| &extract(d).a);
        let mut best_gain_ratio = other.gain_ratio();
        let mut best_branch_width = other.max_branch_width();
        let mut best_decision = __TestStructDataDecision::A(other);

        let other = BranchBuilder::find_best_decision(entropy, data, |d| &extract(d).b);
        if (best_gain_ratio < other.gain_ratio()) || (best_gain_ratio == other.gain_ratio() && best_branch_width > other.max_branch_width()) {
            best_gain_ratio = other.gain_ratio();
            best_branch_width = other.max_branch_width();
            best_decision = __TestStructDataDecision::B(other);
        }

        return best_decision;
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
    fn gain_ratio(&self) -> f64 {
        return match self {
            __TestStructDataDecision::A(a) => a.gain_ratio(),
            __TestStructDataDecision::B(b) => b.gain_ratio(),
        };
    }

    fn max_branch_width(&self) -> usize {
        return match self {
            __TestStructDataDecision::A(a) => a.max_branch_width(),
            __TestStructDataDecision::B(b) => b.max_branch_width(),
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

    fn find_best_decision<R: ToTokens + Copy + Eq + Hash, F, D>(entropy: f64, data: &mut [(D, R)], extract: F) -> Self::Decision
    where F: Fn(&D) -> &Self {
        let other = BranchBuilder::find_best_decision(entropy, data, |d| &extract(d).a);
        let mut best_gain_ratio = other.gain_ratio();
        let mut best_branch_width = other.max_branch_width();
        let mut best_decision = __InnerDecision::A(other);

        println!("Inner::find_best_decision a:{best_gain_ratio}, {best_branch_width}");

        let other = BranchBuilder::find_best_decision(entropy, data, |d| &extract(d).b);
        if (best_gain_ratio < other.gain_ratio()) || (best_gain_ratio == other.gain_ratio() && best_branch_width > other.max_branch_width()) {
            best_gain_ratio = other.gain_ratio();
            best_branch_width = other.max_branch_width();
            best_decision = __InnerDecision::B(other);
        }

        return best_decision;
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
    B(<bool as BranchBuilder>::Decision),
}

impl Decision for __InnerDecision {
    fn gain_ratio(&self) -> f64 {
        return match self {
            __InnerDecision::A(a) => a.gain_ratio(),
            __InnerDecision::B(b) => b.gain_ratio(),
        };
    }

    fn max_branch_width(&self) -> usize {
        return match self {
            __InnerDecision::A(a) => a.max_branch_width(),
            __InnerDecision::B(b) => b.max_branch_width(),
        };
    }

    fn to_condition(&self, var: TokenStream) -> TokenStream {
        return match self {
            __InnerDecision::A(a) => a.to_condition(quote!(#var.a)),
            __InnerDecision::B(b) => b.to_condition(quote!(#var.b)),
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::playground2::TreeBuilder;
    use crate::playground3::{Inner, TestStructData};

    #[test]
    fn test_bool() {
        let mut data = [
            (TestStructData { a: Inner { a: true, b: true }, b: Inner { a: false, b: true } }, true),
            (TestStructData { a: Inner { a: true, b: true }, b: Inner { a: true, b: true } }, false),
        ];
        let decision = TreeBuilder::default().build(&mut data).unwrap();
        println!("{:?}", decision.to_string());
    }
}
