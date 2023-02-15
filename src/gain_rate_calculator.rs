use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

pub struct GainRatioCalculator {
    total: usize,
    entropy: f64,
}

impl GainRatioCalculator {
    pub fn from_results<T>(results: &HashMap<T, usize>) -> Self {
        let total = results.values().sum();

        let mut entropy = 0.0;
        for entry in results.values() {
            entropy += h(*entry, total);
        }

        return GainRatioCalculator { total, entropy };
    }

    pub fn calculate_gain_ratio_bool<T>(&self, data: &HashMap<bool, HashMap<T, usize>>) -> f64 {
        let mut info = 0.0;
        let mut split = vec![];

        for sub_results in data.values() {
            let sum = sub_results.values().sum();
            let mut i = 0.0;
            for count in sub_results.values() {
                i += h(*count, sum);
            }
            info += i * sum as f64 / self.total as f64;
            split.push(sum);
        }

        let mut split_info = 0.0;
        for f in split {
            split_info += h(f, self.total);
        }

        let gain = self.entropy - info;

        return gain / split_info;
    }

    pub fn calculate_gain_ratio_ord<A: Ord + Copy + Hash, T: Hash + Eq>(&self, data: &HashMap<A, HashMap<T, usize>>) -> (f64, A) {
        let mut vals = data.keys().copied().collect::<Vec<A>>();
        vals.sort();

        let mut best_gain_ratio = 0.0;
        let mut best_threshold = vals[0];

        for threshold in &vals[1..] {
            let mut mapped_results = HashMap::new();

            for entry in data {
                let mapped_key = entry.0 < threshold;
                if !mapped_results.contains_key(&mapped_key) {
                    mapped_results.insert(mapped_key, HashMap::new());
                }

                let sub_map = mapped_results.get_mut(&mapped_key).unwrap();

                for outcome in entry.1 {
                    if !sub_map.contains_key(outcome.0) {
                        sub_map.insert(outcome.0, *outcome.1);
                    } else {
                        sub_map.insert(outcome.0, sub_map[&outcome.0] + outcome.1);
                    }
                }
            }

            let gain_ratio = self.calculate_gain_ratio_bool(&mapped_results);

            if gain_ratio > best_gain_ratio {
                best_gain_ratio = gain_ratio;
                best_threshold = *threshold;
            }
        }

        return (best_gain_ratio, best_threshold);
    }
}

fn h(count: usize, total: usize) -> f64 {
    let p = count as f64 / total as f64;
    return -p * p.log2();
}
