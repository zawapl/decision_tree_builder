use std::str::FromStr;

use either::{Either, Left};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::traits::*;

#[derive(Default)]
pub struct TreeBuilder {}

impl TreeBuilder {
    pub fn build<T: TreeBuilderSupport>(&self, data: &mut [(T, T::ResultType)]) -> TokenStream {
        let result_type = format_ident!("{}", std::any::type_name::<T::ResultType>());
        let input_type = TokenStream::from_str(std::any::type_name::<T>()).unwrap();
        let inner = self.build_branch(data);
        return quote!(pub fn decide(val: &#input_type) -> #result_type {
            #inner
        });
    }

    fn build_branch<'a, T: TreeBuilderSupport>(&self, data: &mut [(T, T::ResultType)]) -> TokenStream {
        if data.is_empty() {
            panic!("Ended up in branch with no data to work on");
        }

        let mut gain_calculator = T::GainCalculator::new();

        for i in 0..data.len() {
            gain_calculator.add_entry(&data[i]);
        }

        return match gain_calculator.to_node(data) {
            Left(result) => {
                quote!(#result)
            }
            Either::Right((condition, split)) => {
                let branch_a = self.build_branch(&mut data[..split]);
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
