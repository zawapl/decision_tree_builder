use std::collections::HashMap;
use std::default::Default;
use std::hash::Hash;

use decision_tree_builder_impl::*;
use either::Either;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::{quote, ToTokens};

fn main() {
    let mut data = [
        (TestData { a: 0, b: 0, c: false, d: false }, false),
        (TestData { a: 0, b: 1, c: true, d: true }, true),
        (TestData { a: 1, b: 0, c: false, d: true }, true),
        (TestData { a: 1, b: 1, c: true, d: true }, true),
    ];
    let token_stream = TestData::build_branch(&mut data);
    // let generated_ast = syn::parse2(token_stream).unwrap();
    // let formatted = prettyplease::unparse(&generated_ast);
    println!("{}", token_stream.to_string());
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

impl TestData {
    fn build_branch<R: ToTokens + Copy + Eq + Hash>(data: &mut [(TestData, R)]) -> TokenStream {
        if data.is_empty() {
            panic!("Ended up in branch with no data to work on");
        }

        let mut results = HashMap::new();
        let mut a = HashMap::new();
        let mut b = HashMap::new();

        for i in 0..data.len() {
            let (entry, res) = &data[i];
            *results.entry(*res).or_insert(0) += 1;
            *a.entry(entry.a).or_insert(HashMap::new()).entry(*res).or_insert(0) += 1;
            *b.entry(entry.b).or_insert(HashMap::new()).entry(*res).or_insert(0) += 1;
        }

        if results.keys().count() == 1 {
            let value = *results.keys().next().unwrap();
            return quote!(#value);
        }

        let entropy_calculator = GainRatioCalculator::from_results(&results);

        let gain_ratio_a = entropy_calculator.calculate_gain_ratio_ord(&a);
        let gain_ratio_b = entropy_calculator.calculate_gain_ratio_ord(&b);

        let node = if gain_ratio_b > gain_ratio_a {
            let condition = utils::to_tokens_ord(gain_ratio_b.1, Ident::new("b", Span::call_site()));
            let split = utils::split_data(data, |t| t.0.b < gain_ratio_b.1);
            Either::Right((condition, split))
        } else if gain_ratio_a.0 > 0.0 {
            let condition = utils::to_tokens_ord(gain_ratio_a.1, Ident::new("a", Span::call_site()));
            let split = utils::split_data(data, |t| t.0.a < gain_ratio_a.1);
            Either::Right((condition, split))
        } else {
            let most_common = *results.iter().max_by(|a, b| (a.1).cmp(b.1)).map(|(k, _v)| k).unwrap();
            Either::Left(most_common)
        };

        return match node {
            Either::Left(result) => {
                quote!(#result)
            }
            Either::Right((condition, split)) => {
                let branch_a = Self::build_branch(&mut data[..split]);
                let branch_b = Self::build_branch(&mut data[split..]);

                quote!(if #condition {
                    #branch_a
                } else {
                    #branch_b
                })
            }
        };
    }
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
