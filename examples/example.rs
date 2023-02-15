use std::collections::HashMap;

use decision::{split_data, Decision, GainCalculator, GainRatioCalculator, LeafNode, TreeBuilder, TreeBuilderSupport};
use either::Either;
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote};

fn main() {
    let mut data = [
        TestData { a: 0, b: 0, c: false, d: false },
        TestData { a: 0, b: 1, c: true, d: true },
        TestData { a: 1, b: 0, c: false, d: true },
        TestData { a: 1, b: 1, c: true, d: true },
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
    type GainCalculator = TestDataGainCalculator;
    type ResultType = bool;
}

#[derive(Default)]
pub struct TestDataGainCalculator {
    results: HashMap<bool, usize>,
    a: HashMap<usize, HashMap<bool, usize>>,
    b: HashMap<usize, HashMap<bool, usize>>,
}

impl GainCalculator<TestData, bool> for TestDataGainCalculator {
    type LeafNode = TestDataLeafNode;

    fn add_entry(&mut self, entry: &TestData) {
        *self.results.entry(entry.d).or_insert(0) += 1;
        *self.a.entry(entry.a).or_insert(HashMap::new()).entry(entry.d).or_insert(0) += 1;
        *self.b.entry(entry.b).or_insert(HashMap::new()).entry(entry.d).or_insert(0) += 1;
    }

    fn to_node(self, data: &mut [TestData]) -> Either<Self::LeafNode, (Decision, usize)> {
        if self.results.keys().count() == 1 {
            let value = *self.results.keys().next().unwrap();
            return Either::Left(TestDataLeafNode(value));
        }

        let entropy_calculator = GainRatioCalculator::from_results(&self.results);

        let gain_ratio_a = entropy_calculator.calculate_gain_ratio_ord(&self.a);
        let gain_ratio_b = entropy_calculator.calculate_gain_ratio_ord(&self.b);

        if gain_ratio_b > gain_ratio_a {
            let decision = Decision::UsizeDecision(format_ident!("b"), gain_ratio_b.1);
            let split = split_data(data, |t| t.b < gain_ratio_b.1);
            return Either::Right((decision, split));
        } else if gain_ratio_a.0 > 0.0 {
            let decision = Decision::UsizeDecision(format_ident!("a"), gain_ratio_a.1);
            let split = split_data(data, |t| t.a < gain_ratio_a.1);
            return Either::Right((decision, split));
        } else {
            let most_common = *self.results.iter().max_by(|a, b| (a.1).cmp(b.1)).map(|(k, _v)| k).unwrap();
            return Either::Left(TestDataLeafNode(most_common));
        }
    }
}

#[derive(Default)]
pub struct TestDataLeafNode(bool);

impl LeafNode<bool> for TestDataLeafNode {
    fn get_return_value(&self) -> bool {
        return self.0;
    }
}
