use std::collections::HashMap;

use decision_impl::{utils, GainCalculator, GainRatioCalculator, TreeBuilder, TreeBuilderSupport};
use either::Either;
use proc_macro2::{Ident, Literal, Span, TokenStream};

fn main() {
    let mut data = [
        (TestData { a: 0, b: 0, c: false, d: false }, false),
        (TestData { a: 0, b: 1, c: true, d: true }, true),
        (TestData { a: 1, b: 0, c: false, d: true }, true),
        (TestData { a: 1, b: 1, c: true, d: true }, true),
    ];
    let builder = TreeBuilder::default();
    println!("Result: {}", builder.build(&mut data));
}

#[derive(Default, Debug)]
struct TestData {
    a: usize,
    b: usize,
    c: bool,
    d: bool,
}

impl TreeBuilderSupport for TestData {
    type ResultType = bool;
    type GainCalculator = TestDataGainCalculator;
}

#[derive(Default)]
pub struct TestDataGainCalculator {
    results: HashMap<bool, usize>,
    a: HashMap<usize, HashMap<bool, usize>>,
    b: HashMap<usize, HashMap<bool, usize>>,
}

impl GainCalculator<TestData, bool> for TestDataGainCalculator {
    fn new() -> Self {
        Self::default()
    }

    fn add_entry(&mut self, (entry, res): &(TestData, bool)) {
        *self.results.entry(*res).or_insert(0) += 1;
        *self.a.entry(entry.a).or_insert(HashMap::new()).entry(*res).or_insert(0) += 1;
        *self.b.entry(entry.b).or_insert(HashMap::new()).entry(*res).or_insert(0) += 1;
    }

    fn to_node(self, data: &mut [(TestData, bool)]) -> Either<bool, (TokenStream, usize)> {
        if self.results.keys().count() == 1 {
            let value = *self.results.keys().next().unwrap();
            return Either::Left(value);
        }

        let entropy_calculator = GainRatioCalculator::from_results(&self.results);

        let gain_ratio_a = entropy_calculator.calculate_gain_ratio_ord(&self.a);
        let gain_ratio_b = entropy_calculator.calculate_gain_ratio_ord(&self.b);

        if gain_ratio_b > gain_ratio_a {
            let condition = utils::to_tokens_ord(gain_ratio_b.1, Ident::new("b", Span::call_site()));
            let split = utils::split_data(data, |t| t.0.b < gain_ratio_b.1);
            return Either::Right((condition, split));
        } else if gain_ratio_a.0 > 0.0 {
            let condition = utils::to_tokens_ord(gain_ratio_a.1, Ident::new("a", Span::call_site()));
            let split = utils::split_data(data, |t| t.0.a < gain_ratio_a.1);
            return Either::Right((condition, split));
        } else {
            let most_common = *self.results.iter().max_by(|a, b| (a.1).cmp(b.1)).map(|(k, _v)| k).unwrap();
            return Either::Left(most_common);
        }
    }
}
