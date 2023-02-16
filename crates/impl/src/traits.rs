use either::Either;
use proc_macro2::TokenStream;
use quote::ToTokens;

pub trait TreeBuilderSupport: Sized {
    type ResultType: ToTokens;
    type GainCalculator: GainCalculator<Self, Self::ResultType>;
}

pub trait GainCalculator<T, R> {
    fn new() -> Self;

    fn add_entry(&mut self, entry: &(T, R));

    fn to_node(self, data: &mut [(T, R)]) -> Either<R, (TokenStream, usize)>;
}
