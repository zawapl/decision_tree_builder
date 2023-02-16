use std::marker::PhantomData;
use std::str::FromStr;

use either::{Either, Left};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::traits::*;

#[derive(Default)]
pub struct TreeBuilder {
    add_comments: bool,
}

struct TreeBuilderContext<'a, T> {
    phantom: PhantomData<T>,
    tree_builder: &'a TreeBuilder,
}

impl TreeBuilder {
    pub fn build<T: TreeBuilderSupport>(&self, data: &mut [(T, T::ResultType)]) -> TokenStream {
        let mut builder_context = TreeBuilderContext { phantom: Default::default(), tree_builder: &self };
        let result_type = format_ident!("{}", std::any::type_name::<T::ResultType>());
        let input_type = TokenStream::from_str(std::any::type_name::<T>()).unwrap();
        let inner = builder_context.build_branch(data);
        return quote!(pub fn decide(t: &#input_type) -> #result_type {
            #inner
        });
    }
}

impl<'a, T: TreeBuilderSupport> TreeBuilderContext<'a, T> {
    fn build_branch(&mut self, data: &mut [(T, T::ResultType)]) -> TokenStream {
        if data.is_empty() {
            return quote!(return 0;);
        }

        let mut gain_calculator = T::GainCalculator::new();

        for i in 0..data.len() {
            gain_calculator.add_entry(&data[i]);
        }

        return match gain_calculator.to_node(data) {
            Left(result) => {
                // let result = leaf.get_return_value();
                quote!(#result)
            }
            Either::Right((condition, split)) => {
                println!("Goto branch a");
                let branch_a = self.build_branch(&mut data[..split]);
                println!("Goto branch b");
                let branch_b = self.build_branch(&mut data[split..]);

                quote!(if #condition {
                    #branch_a
                } else {
                    #branch_b
                })
            }
        };
    }
}
