#[macro_export]
macro_rules! ord_implementation {
    ($t:ident) => {
        impl decision_tree_builder_impl::BranchBuilder for $t {
            type Decision = decision_tree_builder_impl::OrdDecision<Self>;

            fn find_best_decision<R: Copy + Eq + std::hash::Hash, F, D>(
                entropy: f64,
                data: &mut [(D, R)],
                extract: F,
            ) -> Self::Decision
            where
                F: Fn(&D) -> &Self,
            {
                let vals: Vec<Self> = data.iter().map(|(d, _)| extract(d)).copied().collect();

                let mut best_gain_ratio = 0.0;
                let mut best_threshold = vals[0];
                let mut best_branch_size = usize::MAX;

                for threshold in &vals[0..] {
                    let total_count = data.len();
                    let mut true_sub_branch = std::collections::HashMap::new();
                    let mut false_sub_branch = std::collections::HashMap::new();

                    for i in 0..data.len() {
                        let (entry, res) = &data[i];
                        let branch = extract(entry) < threshold;
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
                            i += decision_tree_builder_impl::utils::h(*count, sum);
                        }
                        info += i * sum as f64 / total_count as f64;
                        split.push(sum);
                        max_branch_width = max_branch_width.max(sum);
                    }

                    let mut split_info = 0.0;
                    for f in split {
                        split_info += decision_tree_builder_impl::utils::h(f, total_count);
                    }

                    let gain_ratio = if split_info == 0.0 {
                        0.0
                    } else {
                        (entropy - info) / split_info
                    };

                    if (gain_ratio > best_gain_ratio)
                        || (gain_ratio == best_gain_ratio && max_branch_width < best_branch_size)
                    {
                        best_gain_ratio = gain_ratio;
                        best_threshold = *threshold;
                        best_branch_size = max_branch_width;
                    }
                }

                let decision_eval = decision_tree_builder_impl::DecisionEval {
                    gain_ratio: best_gain_ratio,
                    max_branch_width: best_branch_size,
                };
                return Self::Decision { decision_eval, threshold: best_threshold };
            }

            fn split_data<F, D, R>(
                data: &mut [(D, R)],
                extract: F,
                decision: &Self::Decision,
            ) -> usize
            where
                F: Fn(&D) -> &Self,
            {
                return decision_tree_builder_impl::utils::split_data(data, |(d, _)| {
                    extract(d) < &&decision.threshold
                });
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use proc_macro2::TokenStream;
    use quote::{format_ident, quote, ToTokens};

    use crate as decision_tree_builder_impl;
    use crate::ToFormattedTokens;

    #[derive(Clone, Copy, PartialEq, Debug)]
    enum TestEnum {
        A,
        B,
    }

    ord_implementation!(TestEnum);

    impl ToFormattedTokens for TestEnum {
        fn to_formatted_tokens(&self) -> TokenStream {
            return match self {
                TestEnum::A => format_ident!("A").to_token_stream(),
                TestEnum::B => format_ident!("B").to_token_stream(),
            };
        }
    }

    impl PartialOrd for TestEnum {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            let res = match self {
                TestEnum::A => {
                    match other {
                        TestEnum::A => Ordering::Equal,
                        TestEnum::B => Ordering::Greater,
                    }
                }
                TestEnum::B => {
                    match other {
                        TestEnum::A => Ordering::Less,
                        TestEnum::B => Ordering::Equal,
                    }
                }
            };
            return Some(res);
        }
    }

    #[test]
    fn test_enum() {
        let mut data = [(TestEnum::A, 1), (TestEnum::B, 2)];
        let decision = decision_tree_builder_impl::TreeBuilder::default().build(&mut data).unwrap();
        let expected = quote!(
            pub fn decide(
                val: &decision_tree_builder_impl::branch_builder::ord_macro::tests::TestEnum,
            ) -> i32 {
                return if val < A { 2 } else { 1 };
            }
        );
        assert_eq!(decision.to_string(), expected.to_string());
    }
}
