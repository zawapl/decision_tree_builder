use either::Either;
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, ToTokens};

use crate::decision::Decision;

pub trait TreeBuilderSupport: Sized {
    type GainCalculator: GainCalculator<Self, Self::ResultType>;
    type ResultType: ToTokens;
}

pub trait GainCalculator<T, R>: Default {
    type LeafNode: LeafNode<R>;

    fn add_entry(&mut self, entry: &T);

    fn to_node(self, data: &mut [T]) -> Either<Self::LeafNode, (Decision, usize)>;
}

pub trait LeafNode<R> {
    fn get_return_value(&self) -> R;
}
