use decision_tree_builder_impl::TreeBuilder;

pub struct TestData {
    a: usize,
    b: usize,
    c: bool,
    d: bool,
}

fn main() {
    let mut test_data = [
        (TestData { a: 0, b: 0, c: false, d: false }, false),
        (TestData { a: 0, b: 1, c: false, d: true }, true),
        (TestData { a: 1, b: 0, c: false, d: true }, true),
        (TestData { a: 1, b: 1, c: false, d: false }, false),
    ];
    let token_stream = TreeBuilder::default().build(&mut test_data);
    let generated_ast = syn::parse2(token_stream).unwrap();
    let formatted = prettyplease::unparse(&generated_ast);
    println!("{}", formatted);
}

pub mod tree_builder_support {
    use std::collections::HashMap;
    use std::hash::Hash;

    use decision_tree_builder_impl::*;
    use either::Either;
    use proc_macro2::{Ident, Literal, Span, TokenStream};

    use super::TestData;
    impl<R: Copy + Default + Eq + Hash> TreeBuilderSupport<R> for TestData {
        type GainCalculator<T> = TestDataGainCalculator<R>;
    }
    pub struct TestDataGainCalculator {
        __results: HashMap<R, usize>,
        a: HashMap<usize, HashMap<bool, usize>>,
        b: HashMap<usize, HashMap<bool, usize>>,
        c: HashMap<bool, HashMap<bool, usize>>,
        d: HashMap<bool, HashMap<bool, usize>>,
    }
    impl<R: Copy + Default + Eq + Hash> GainCalculator<TestData, R> for TestDataGainCalculator<R> {
        fn new() -> Self {
            return TestDataGainCalculator { __results: HashMap::new(), a: HashMap::new(), b: HashMap::new(), c: HashMap::new(), d: HashMap::new() };
        }
        fn add_entry(&mut self, (entry, res): &(TestData, R)) {
            *self.__results.entry(*res).or_insert(0) += 1;
            *self.a.entry(entry.a).or_insert(HashMap::new()).entry(*res).or_insert(0) += 1;
            *self.b.entry(entry.b).or_insert(HashMap::new()).entry(*res).or_insert(0) += 1;
            *self.c.entry(entry.c).or_insert(HashMap::new()).entry(*res).or_insert(0) += 1;
            *self.d.entry(entry.d).or_insert(HashMap::new()).entry(*res).or_insert(0) += 1;
        }
        fn to_node(self, data: &mut [(TestData, R)]) -> Either<R, (TokenStream, usize)> {
            let gain_ratio_calculator = GainRatioCalculator::from_results(&self.__results);
            let mut best_gain = 0.0;
            let mut best_field = usize::MAX;
            let mut best_branch_size = usize::MAX;
            let a = gain_ratio_calculator.calculate_gain_ratio_ord(&self.a);
            if (a.0 > best_gain) || (a.0 == best_gain && a.2 < best_branch_size) {
                best_gain = a.0;
                best_field = 0usize;
                best_branch_size = a.2;
            }
            let b = gain_ratio_calculator.calculate_gain_ratio_ord(&self.b);
            if (b.0 > best_gain) || (b.0 == best_gain && b.2 < best_branch_size) {
                best_gain = b.0;
                best_field = 1usize;
                best_branch_size = b.2;
            }
            let c = gain_ratio_calculator.calculate_gain_ratio_bool(&self.c);
            if (c.0 > best_gain) || (c.0 == best_gain && c.2 < best_branch_size) {
                best_gain = c.0;
                best_field = 2usize;
                best_branch_size = c.2;
            }
            let d = gain_ratio_calculator.calculate_gain_ratio_bool(&self.d);
            if (d.0 > best_gain) || (d.0 == best_gain && d.2 < best_branch_size) {
                best_gain = d.0;
                best_field = 3usize;
                best_branch_size = d.2;
            }
            match best_field {
                3usize => {
                    let condition = utils::to_tokens_eq(d.1, Ident::new("d", Span::call_site()));
                    let split = utils::split_data(data, |t| t.0.d == d.1);
                    return Either::Right((condition, split));
                }
                2usize => {
                    let condition = utils::to_tokens_eq(c.1, Ident::new("c", Span::call_site()));
                    let split = utils::split_data(data, |t| t.0.c == c.1);
                    return Either::Right((condition, split));
                }
                1usize => {
                    let literal = Literal::usize_unsuffixed(b.1);
                    let condition = utils::to_tokens_ord(literal, Ident::new("b", Span::call_site()));
                    let split = utils::split_data(data, |t| t.0.b < b.1);
                    return Either::Right((condition, split));
                }
                0usize => {
                    let literal = Literal::usize_unsuffixed(a.1);
                    let condition = utils::to_tokens_ord(literal, Ident::new("a", Span::call_site()));
                    let split = utils::split_data(data, |t| t.0.a < a.1);
                    return Either::Right((condition, split));
                }
                _ => {
                    let most_common = *self.__results.iter().max_by(|a, b| (a.1).cmp(b.1)).map(|(k, _v)| k).unwrap();
                    return Either::Left(most_common);
                }
            }
        }
    }
}
