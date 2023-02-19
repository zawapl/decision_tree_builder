use std::collections::HashMap;
use std::hash::Hash;

use crate as decision_tree_builder_impl;
use crate::decision_eval::DecisionEval;
use crate::{eq_implementation, ord_implementation, utils, ArrayDecision, BoolDecision, Decision, Tuple2Decision};

///
pub trait BranchBuilder {
    ///
    type Decision: Decision;

    ///
    fn find_best_decision<R: Copy + Eq + Hash, F, D>(entropy: f64, data: &mut [(D, R)], extract: F) -> Self::Decision
    where F: Fn(&D) -> &Self;

    ///
    fn split_data<F, D, R>(data: &mut [(D, R)], extract: F, decision: &Self::Decision) -> usize
    where F: Fn(&D) -> &Self;
}

type StaticStr = &'static str;
eq_implementation!(String);
eq_implementation!(StaticStr);

ord_implementation!(u8);
ord_implementation!(u16);
ord_implementation!(u32);
ord_implementation!(u64);
ord_implementation!(u128);
ord_implementation!(usize);

ord_implementation!(i8);
ord_implementation!(i16);
ord_implementation!(i32);
ord_implementation!(i64);
ord_implementation!(i128);
ord_implementation!(isize);

ord_implementation!(f32);
ord_implementation!(f64);

/// Support for String
impl BranchBuilder for bool {
    type Decision = BoolDecision;

    fn find_best_decision<R: Copy + Eq + Hash, F, D>(entropy: f64, data: &mut [(D, R)], extract: F) -> Self::Decision
    where F: Fn(&D) -> &Self {
        let total_count = data.len();
        let mut true_sub_branch = HashMap::new();
        let mut false_sub_branch = HashMap::new();

        for i in 0..data.len() {
            let (entry, res) = &data[i];
            let branch = *extract(entry);
            if branch {
                *true_sub_branch.entry(*res).or_insert(0) += 1;
            } else {
                *false_sub_branch.entry(*res).or_insert(0) += 1;
            }
        }

        let mut info = 0.0;
        let mut split = vec![];
        let mut max_branch_width = 0;

        for sub_results in [true_sub_branch, false_sub_branch] {
            let sum = sub_results.values().sum();
            let mut i = 0.0;
            for count in sub_results.values() {
                i += utils::h(*count, sum);
            }
            info += i * sum as f64 / total_count as f64;
            split.push(sum);
            max_branch_width = max_branch_width.max(sum);
        }

        let mut split_info = 0.0;
        for f in split {
            split_info += utils::h(f, total_count);
        }

        let gain_ratio = if split_info == 0.0 {
            0.0
        } else {
            (entropy - info) / split_info
        };

        let decision_eval = DecisionEval { gain_ratio, max_branch_width };
        return BoolDecision { decision_eval };
    }

    fn split_data<F, D, R>(data: &mut [(D, R)], extract: F, _decision: &Self::Decision) -> usize
    where F: Fn(&D) -> &Self {
        return utils::split_data(data, |(d, _)| *extract(d));
    }
}

/// Support for tuples
impl<A, B> BranchBuilder for (A, B)
where
    A: BranchBuilder,
    B: BranchBuilder,
{
    type Decision = Tuple2Decision<A::Decision, B::Decision>;

    fn find_best_decision<R: Copy + Eq + Hash, F, D>(entropy: f64, data: &mut [(D, R)], extract: F) -> Self::Decision
    where F: Fn(&D) -> &Self {
        return [
            Tuple2Decision::A(BranchBuilder::find_best_decision(entropy, data, |d| &extract(d).0)),
            Tuple2Decision::B(BranchBuilder::find_best_decision(entropy, data, |d| &extract(d).1)),
        ]
        .into_iter()
        .max_by(|a, b| a.to_decision_eval().cmp(b.to_decision_eval()))
        .unwrap();
    }

    fn split_data<F, D, R>(data: &mut [(D, R)], extract: F, decision: &Self::Decision) -> usize
    where F: Fn(&D) -> &Self {
        return match decision {
            Tuple2Decision::A(a) => BranchBuilder::split_data(data, |d| &extract(d).0, a),
            Tuple2Decision::B(b) => BranchBuilder::split_data(data, |d| &extract(d).1, b),
        };
    }
}


/// Support for arrays
impl<T, const N: usize> BranchBuilder for [T; N]
where T: BranchBuilder
{
    type Decision = ArrayDecision<T::Decision>;

    fn find_best_decision<R: Copy + Eq + Hash, F, D>(entropy: f64, data: &mut [(D, R)], extract: F) -> Self::Decision
    where F: Fn(&D) -> &Self {
        return (0..N)
            .map(|i| ArrayDecision { index: i, inner_decision: BranchBuilder::find_best_decision(entropy, data, |d| &extract(d)[i]) })
            .max_by(|a, b| a.to_decision_eval().cmp(b.to_decision_eval()))
            .unwrap();
    }

    fn split_data<F, D, R>(data: &mut [(D, R)], extract: F, decision: &Self::Decision) -> usize
    where F: Fn(&D) -> &Self {
        return BranchBuilder::split_data(data, |d| &extract(d)[decision.index], &decision.inner_decision);
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use crate::*;

    #[test]
    fn test_bool() {
        let mut data = [(true, true), (false, false)];
        let decision = BranchBuilder::find_best_decision(utils::entropy(&utils::to_counts(&data)), &mut data[..], |v| v);
        let condition = decision.to_condition(quote!(val));
        let expected = quote!(val);
        assert_eq!(condition.to_string(), expected.to_string());
        assert_eq!(decision.to_decision_eval().max_branch_width, 1);
        assert_eq!(decision.to_decision_eval().gain_ratio, 1.0);
    }

    #[test]
    fn test_bool_ref() {
        let mut data = [(&true, true), (&false, false), (&false, false)];
        let decision = BranchBuilder::find_best_decision(utils::entropy(&utils::to_counts(&data)), &mut data[..], |v| *v);
        let condition = decision.to_condition(quote!(val));
        let expected = quote!(val);
        assert_eq!(condition.to_string(), expected.to_string());
        assert_eq!(decision.to_decision_eval().max_branch_width, 1);
        assert_eq!(decision.to_decision_eval().gain_ratio, 1.0);
    }

    #[test]
    fn test_tuple() {
        let mut data = [((0, 0), true), ((0, 1), false), ((1, 0), false)];
        let decision = BranchBuilder::find_best_decision(utils::entropy(&utils::to_counts(&data)), &mut data[..], |v| v);
        let condition = decision.to_condition(quote!(val));
        let expected = quote!(*val.1 < 1);
        assert_eq!(condition.to_string(), expected.to_string());
        assert_eq!(decision.to_decision_eval().max_branch_width, 2);
        assert_eq!(decision.to_decision_eval().gain_ratio, 0.5);
    }

    #[test]
    fn test_recursive_tuple() {
        let mut data = [(((0, 0), (true, true)), true), (((0, 1), (false, false)), false), (((1, 0), (true, true)), false), (((1, 1), (false, false)), false)];
        let original_entropy = utils::entropy(&utils::to_counts(&data));
        let decision = BranchBuilder::find_best_decision(original_entropy, &mut data[..], |v| v);
        let condition = decision.to_condition(quote!(val));
        let expected = quote!(val.1 .1);
        assert_eq!(condition.to_string(), expected.to_string());
        assert_eq!(decision.to_decision_eval().max_branch_width, 2);

        // let expected_gain_ratio = original_entropy -
        assert_eq!(decision.to_decision_eval().gain_ratio, 0.5);
    }

    #[test]
    fn test_list() {
        let mut data = [([true, true], true), ([true, false], false), ([false, true], false), ([false, false], true)];
        let decision = BranchBuilder::find_best_decision(utils::entropy(&utils::to_counts(&data)), &mut data[..], |v| v);
        let condition = decision.to_condition(quote!(val));
        let expected = quote!(val[1]);
        assert_eq!(condition.to_string(), expected.to_string());
        assert_eq!(decision.to_decision_eval().max_branch_width, 2);
        assert_eq!(decision.to_decision_eval().gain_ratio, 0.0);
    }

    /// Using numbers form https://sefiks.com/2018/05/13/a-step-by-step-c4-5-decision-tree-example/
    #[test]
    fn test_example() {
        let mut data = [
            ("Weak", false),
            ("Strong", false),
            ("Weak", true),
            ("Weak", true),
            ("Weak", true),
            ("Strong", false),
            ("Strong", true),
            ("Weak", false),
            ("Weak", true),
            ("Weak", true),
            ("Strong", true),
            ("Strong", true),
            ("Weak", true),
            ("Strong", false),
        ];
        let decision = BranchBuilder::find_best_decision(utils::entropy(&utils::to_counts(&data)), &mut data[..], |v| v);
        let condition = decision.to_condition(quote!(val));
        let expected = quote!(val == "Weak");
        assert_eq!(condition.to_string(), expected.to_string());
        assert_eq!(decision.to_decision_eval().max_branch_width, 2);
        assert_eq!(decision.to_decision_eval().gain_ratio, 0.04884861551152088);
    }
}
