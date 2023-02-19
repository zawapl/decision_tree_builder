#[macro_export]
macro_rules! eq_implementation {
    ($t:ident) => {
        impl BranchBuilder for $t {
            type Decision = EqDecision<$t>;

            fn find_best_decision<R: ToTokens + Copy + Eq + Hash, F, D>(entropy: f64, data: &mut [(D, R)], extract: F) -> Self::Decision
            where F: Fn(&D) -> &Self {
                let total_count = data.len();
                let mut true_sub_branch = HashMap::new();
                let mut false_sub_branch = HashMap::new();

                for i in 0..data.len() {
                    let (entry, res) = &data[i];
                    let branch = *extract(entry) == true;
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

                return EqDecision { gain_ratio, max_branch_width };
            }

            fn split_data<F, D, R>(data: &mut [(D, R)], extract: F, _decision: &Self::Decision) -> usize
            where F: Fn(&D) -> &Self {
                return utils::split_data(data, |(d, _)| *extract(d));
            }
        }
    };
}
