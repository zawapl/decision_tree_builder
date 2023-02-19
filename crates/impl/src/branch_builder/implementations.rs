use std::collections::HashMap;
use std::hash::Hash;

use quote::ToTokens;

use crate::branch_builder::num_macro;
use crate::{eq_implementation, num_implementation, utils, ArrayDecision, BoolDecision, Decision, EqDecision, OrdDecision, Tuple2Decision};

///
pub trait BranchBuilder {
    ///
    type Decision: Decision;

    ///
    fn find_best_decision<R: ToTokens + Copy + Eq + Hash, F, D>(entropy: f64, data: &mut [(D, R)], extract: F) -> Self::Decision
    where F: Fn(&D) -> &Self;

    ///
    fn split_data<F, D, R>(data: &mut [(D, R)], extract: F, decision: &Self::Decision) -> usize
    where F: Fn(&D) -> &Self;
}


// eq_implementation!(bool);
num_implementation!(usize);
num_implementation!(u32);
num_implementation!(f32);

/// Support for String
impl BranchBuilder for bool {
    type Decision = BoolDecision;

    fn find_best_decision<R: ToTokens + Copy + Eq + Hash, F, D>(entropy: f64, data: &mut [(D, R)], extract: F) -> Self::Decision
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
            max_branch_width = max_branch_width.max(sub_results.len());
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

        return BoolDecision { gain_ratio, max_branch_width };
    }

    fn split_data<F, D, R>(data: &mut [(D, R)], extract: F, _decision: &Self::Decision) -> usize
    where F: Fn(&D) -> &Self {
        return utils::split_data(data, |(d, _)| *extract(d));
    }
}


// impl BranchBuilder for String {
//     type Decision = EqDecision<String>;
//
//     fn find_best_decision<R: ToTokens + Copy + Eq + Hash, F, D>(entropy: f64, data: &mut [(D, R)], extract: F) -> Self::Decision
//     where F: Fn(&D) -> &Self {
//         let vals: Vec<&String> = data.iter().map(|(d, _)| extract(d)).copied().collect();
//
//         let mut best_gain_ratio = 0.0;
//         let mut best_threshold = &vals[0];
//         let mut best_branch_size = usize::MAX;
//
//         for threshold in &vals[0..] {
//             let total_count = data.len();
//             let mut true_sub_branch = HashMap::new();
//             let mut false_sub_branch = HashMap::new();
//
//             for i in 0..data.len() {
//                 let (entry, res) = &data[i];
//                 let branch = extract(entry) < threshold;
//                 if branch {
//                     *true_sub_branch.entry(*res).or_insert(0) += 1;
//                 } else {
//                     *false_sub_branch.entry(*res).or_insert(0) += 1;
//                 }
//             }
//
//             let mut info = 0.0;
//             let mut split = vec![];
//             let mut max_branch_width = 0;
//
//             for sub_results in [true_sub_branch, false_sub_branch] {
//                 let sum = sub_results.values().sum();
//                 let mut i = 0.0;
//                 for count in sub_results.values() {
//                     i += utils::h(*count, sum);
//                 }
//                 info += i * sum as f64 / total_count as f64;
//                 split.push(sum);
//                 max_branch_width = max_branch_width.max(sub_results.len());
//             }
//
//             let mut split_info = 0.0;
//             for f in split {
//                 split_info += utils::h(f, total_count);
//             }
//
//             let gain_ratio = if split_info == 0.0 {
//                 0.0
//             } else {
//                 (entropy - info) / split_info
//             };
//
//             if (gain_ratio > best_gain_ratio) || (gain_ratio == best_gain_ratio && max_branch_width < best_branch_size) {
//                 best_gain_ratio = gain_ratio;
//                 best_threshold = threshold;
//                 best_branch_size = max_branch_width;
//             }
//         }
//
//         return EqDecision { gain_ratio: best_gain_ratio, val: best_threshold, max_branch_width: best_branch_size };
//     }
//
//     fn split_data<F, D, R>(data: &mut [(D, R)], extract: F, _decision: &Self::Decision) -> usize
//     where F: Fn(&D) -> &Self {
//         return utils::split_data(data, |(d, _)| *extract(d));
//     }
// }

/// Support for tuples
impl<A, B> BranchBuilder for (A, B)
where
    A: BranchBuilder,
    B: BranchBuilder,
{
    type Decision = Tuple2Decision<A::Decision, B::Decision>;

    fn find_best_decision<R: ToTokens + Copy + Eq + Hash, F, D>(entropy: f64, data: &mut [(D, R)], extract: F) -> Self::Decision
    where F: Fn(&D) -> &Self {
        let a = BranchBuilder::find_best_decision(entropy, data, |d| &extract(d).0);
        let b = BranchBuilder::find_best_decision(entropy, data, |d| &extract(d).1);

        return if a.gain_ratio() > b.gain_ratio() {
            Tuple2Decision::A(a)
        } else {
            Tuple2Decision::B(b)
        };
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

    fn find_best_decision<R: ToTokens + Copy + Eq + Hash, F, D>(entropy: f64, data: &mut [(D, R)], extract: F) -> Self::Decision
    where F: Fn(&D) -> &Self {
        let mut best_decision = BranchBuilder::find_best_decision(entropy, data, |d| &extract(d)[0]);
        let mut best_i = 0;
        for i in 1..N {
            let decision = BranchBuilder::find_best_decision(entropy, data, |d| &extract(d)[i]);
            let gain_ratio = decision.gain_ratio();
            let best_gain_ratio = best_decision.gain_ratio();
            if (gain_ratio > best_gain_ratio) || (gain_ratio == best_gain_ratio && decision.max_branch_width() < best_decision.max_branch_width()) {
                best_decision = decision;
                best_i = i;
            }
        }
        return ArrayDecision { index: best_i, inner_decision: best_decision };
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
        let mut data = [(true, true), (false, false), (false, false), (false, false), (false, false)];
        let decision = BranchBuilder::find_best_decision(1.0, &mut data[..], |v| v);
        println!("{:?}", decision);
    }

    #[test]
    fn test_ref_bool() {
        let mut data = [(&true, true), (&false, false), (&false, false), (&false, false), (&false, false)];
        let decision = BranchBuilder::find_best_decision(1.0, &mut data[..], |v| *v);
        println!("{:?}", decision);
    }

    #[test]
    fn test_usize() {
        let mut data = [(0usize, true), (1, true), (2, false), (3, false), (4, false)];
        let decision = BranchBuilder::find_best_decision(1.0, &mut data[..], |v| v);
        println!("{:?}", decision);
        println!("{:?}", decision.to_condition(quote!(self)).to_string());
    }

    #[test]
    fn test_tuple() {
        let mut data = [((0, 0), true), ((0, 1), false), ((1, 0), false), ((1, 1), false)];
        let decision = BranchBuilder::find_best_decision(1.0, &mut data[..], |v| v);
        println!("{:?}", decision);
        println!("{:?}", decision.to_condition(quote!(self)).to_string());
    }

    #[test]
    fn test_recursive_tuple() {
        let mut data = [(((0, 0), (true, true)), true), (((0, 1), (false, false)), false), (((1, 0), (true, true)), false), (((1, 1), (false, false)), false)];
        let decision = BranchBuilder::find_best_decision(1.0, &mut data[..], |v| v);
        println!("{:?}", decision);
        println!("{:?}", decision.to_condition(quote!(self)).to_string());
    }

    #[test]
    fn test_list() {
        let mut data = [([true, true], true), ([true, false], false), ([false, true], false), ([false, false], true)];
        let decision = BranchBuilder::find_best_decision(1.0, &mut data[..], |v| v);
        println!("{:?}", decision);
        println!("{:?}", decision.to_condition(quote!(self)).to_string());
    }
}
