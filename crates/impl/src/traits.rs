use either::Either;
use proc_macro2::TokenStream;


pub trait TreeBuilderSupport<R>: Sized {
    type GainCalculator<T>: GainCalculator<Self, R>;
}

pub trait GainCalculator<T, R> {
    fn new() -> Self;

    fn add_entry(&mut self, entry: &(T, R));

    fn to_node(self, data: &mut [(T, R)]) -> Either<R, (TokenStream, usize)>;
}
