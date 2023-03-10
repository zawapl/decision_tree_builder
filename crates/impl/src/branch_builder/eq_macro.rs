#[macro_export]
macro_rules! eq_implementation {
    ($t:ident) => {
        impl decision_tree_builder_impl::BranchBuilder for $t {
            type Decision = decision_tree_builder_impl::EqDecision<Self>;

            fn find_best_decision<R: Copy + Eq + std::hash::Hash, F, D>(entropy_decision: f64, data: &mut [(D, R)], extract: F) -> Self::Decision
            where F: Fn(&D) -> &Self {
                let mut vals: Vec<Self> = vec![];

                for (d, _) in data.iter() {
                    let val = extract(d);
                    if !vals.contains(val) {
                        vals.push(val.clone());
                    }
                }

                let mut best_gain_ratio = 0.0;
                let mut best_val = &vals[0];
                let mut best_branch_size = usize::MAX;

                for test_val in &vals[0..] {
                    let total_count = data.len();
                    let mut true_sub_branch = std::collections::HashMap::new();
                    let mut false_sub_branch = std::collections::HashMap::new();

                    for i in 0..data.len() {
                        let (entry, res) = &data[i];
                        let branch = extract(entry) == test_val;
                        if branch {
                            *true_sub_branch.entry(*res).or_insert(0) += 1;
                        } else {
                            *false_sub_branch.entry(*res).or_insert(0) += 1;
                        }
                    }

                    let mut gain = entropy_decision;
                    let mut split = vec![];
                    let mut max_branch_width = 0;

                    for sub_results in [true_sub_branch, false_sub_branch] {
                        let sum = sub_results.values().sum();
                        let mut i = 0.0;
                        for count in sub_results.values() {
                            i += decision_tree_builder_impl::utils::h(*count, sum);
                        }
                        gain -= i * sum as f64 / total_count as f64;
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
                        gain / split_info
                    };

                    if (gain_ratio > best_gain_ratio) || (gain_ratio == best_gain_ratio && max_branch_width < best_branch_size) {
                        best_gain_ratio = gain_ratio;
                        best_val = test_val;
                        best_branch_size = max_branch_width;
                    }
                }

                let decision_eval = decision_tree_builder_impl::DecisionEval { gain_ratio: best_gain_ratio, max_branch_width: best_branch_size };
                return Self::Decision { val: best_val.clone(), decision_eval };
            }

            fn split_data<F, D, R>(data: &mut [(D, R)], extract: F, decision: &Self::Decision) -> usize
            where F: Fn(&D) -> &Self {
                return decision_tree_builder_impl::utils::split_data(data, |(d, _)| extract(d) == &decision.val);
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;
    use quote::{format_ident, quote, ToTokens, TokenStreamExt};

    use crate as decision_tree_builder_impl;

    #[derive(Clone, PartialEq)]
    enum TestEnum {
        A,
        B,
    }

    eq_implementation!(TestEnum);

    impl ToTokens for TestEnum {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match self {
                TestEnum::A => tokens.append(format_ident!("A")),
                TestEnum::B => tokens.append(format_ident!("B")),
            };
        }
    }

    #[test]
    fn test_enum() {
        let mut data = [(TestEnum::A, 1), (TestEnum::B, 2)];
        let decision = decision_tree_builder_impl::TreeBuilder::default().build(&mut data).unwrap();
        let expected = quote!(
            pub fn decide(val: &decision_tree_builder_impl::branch_builder::eq_macro::tests::TestEnum) -> i32 {
                return if val == A { 1 } else { 2 };
            }
        );
        assert_eq!(decision.to_string(), expected.to_string());
    }
}
