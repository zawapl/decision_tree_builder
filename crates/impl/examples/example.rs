use std::collections::HashMap;
use std::default::Default;
use std::hash::Hash;

use decision_tree_builder_impl::*;
use either::Either;
use proc_macro2::{Ident, Literal, Span, TokenStream};

fn main() {
    let mut data = [
        (TestData { a: 0, b: 0, c: false, d: false }, false),
        (TestData { a: 0, b: 1, c: true, d: true }, true),
        (TestData { a: 1, b: 0, c: false, d: true }, true),
        (TestData { a: 1, b: 1, c: true, d: true }, true),
    ];
    let token_stream = TreeBuilder::default().build(&mut data);
    let generated_ast = syn::parse2(token_stream).unwrap();
    let formatted = prettyplease::unparse(&generated_ast);
    println!("{}", formatted);
}

#[derive(Default, Debug)]
struct TestData {
    a: usize,
    b: usize,
    c: bool,
    d: bool,
}

impl<R: Copy + Default + Eq + Hash> TreeBuilderSupport<R> for TestData {
    type GainCalculator<T> = TestDataGainCalculator<R>;
}

pub struct TestDataGainCalculator<R> {
    results: HashMap<R, usize>,
    a: HashMap<usize, HashMap<R, usize>>,
    b: HashMap<usize, HashMap<R, usize>>,
}

impl<R: Copy + Eq + Hash> GainCalculator<TestData, R> for TestDataGainCalculator<R> {
    fn new() -> Self {
        TestDataGainCalculator { results: HashMap::new(), a: HashMap::new(), b: HashMap::new() }
    }

    fn add_entry(&mut self, (entry, res): &(TestData, R)) {
        *self.results.entry(*res).or_insert(0) += 1;
        *self.a.entry(entry.a).or_insert(HashMap::new()).entry(*res).or_insert(0) += 1;
        *self.b.entry(entry.b).or_insert(HashMap::new()).entry(*res).or_insert(0) += 1;
    }

    fn to_node(self, data: &mut [(TestData, R)]) -> Either<R, (TokenStream, usize)> {
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
