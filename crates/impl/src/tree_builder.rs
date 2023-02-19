use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;

use proc_macro2::{LexError, Literal, TokenStream};
use quote::{quote, ToTokens};

use crate::{utils, BranchBuilder, Decision};

pub struct TreeBuilder {
    max_depth: usize,
    explain_uncertain_leaves: bool,
}

pub type TreeBuilderError = LexError;

struct TreeBuilderContext {
    depth: usize,
}

impl TreeBuilderContext {
    fn new() -> Self {
        return TreeBuilderContext { depth: 10 };
    }

    fn next(&self) -> Self {
        return TreeBuilderContext { depth: self.depth + 1 };
    }
}

impl Default for TreeBuilder {
    fn default() -> Self {
        let max_depth = 100;
        let explain_uncertain_leaves = true;
        return TreeBuilder { max_depth, explain_uncertain_leaves };
    }
}

impl TreeBuilder {
    pub fn build<T, R: ToTokens + Copy + Eq + Hash>(&self, data: &mut [(T, R)]) -> Result<TokenStream, TreeBuilderError>
    where T: BranchBuilder {
        let result_type = TokenStream::from_str(std::any::type_name::<R>())?;
        let input_type = TokenStream::from_str(std::any::type_name::<T>())?;
        let context = TreeBuilderContext::new();
        let inner = self.build_branch(&context, data)?;
        return Ok(quote!(pub fn decide(val: &#input_type) -> #result_type {
            #inner
        }));
    }

    fn build_branch<T, R: ToTokens + Copy + Eq + Hash>(&self, context: &TreeBuilderContext, data: &mut [(T, R)]) -> Result<TokenStream, TreeBuilderError>
    where T: BranchBuilder {
        let mut results = HashMap::new();

        for (_, res) in data.iter() {
            *results.entry(*res).or_insert(0) += 1;
        }

        // If there is only one possible result left in this branch
        // or if max depth has been reached
        if results.len() == 1 || context.depth > self.max_depth {
            return Ok(self.get_most_common_result(&results));
        }

        let total = results.values().sum();

        let mut entropy = 0.0;
        for entry in results.values() {
            entropy += utils::h(*entry, total);
        }

        let decision = BranchBuilder::find_best_decision(entropy, &mut data[..], |v| v);

        // If there is no gain
        if decision.gain_ratio() == 0.0 {
            return Ok(self.get_most_common_result(&results));
        }

        let condition = decision.to_condition(TokenStream::from_str("val")?);

        let split = BranchBuilder::split_data(&mut data[..], |v| v, &decision);

        let next_context = context.next();
        let branch_a = self.build_branch(&next_context, &mut data[..split])?;
        let branch_b = self.build_branch(&next_context, &mut data[split..])?;

        return Ok(quote!(
            return if #condition {
                #branch_a
            } else {
                #branch_b
            }
        ));
    }

    fn get_most_common_result<R: Eq + ToTokens>(&self, map: &HashMap<R, usize>) -> TokenStream {
        return if map.len() == 1 {
            let result = map.keys().next().unwrap();
            quote!(#result)
        } else {
            let result = map.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0.to_token_stream();
            let mut comment = TokenStream::new();
            if self.explain_uncertain_leaves {
                for (r, c) in map {
                    let count = Literal::usize_unsuffixed(*c);
                    comment = quote!(#comment #[tree_builder_uncertain_node(#r = #count)])
                }
            }
            quote!(#comment #result)
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::tree_builder::TreeBuilder;

    #[test]
    fn test_bool() {
        let mut data = [(0, true), (0, false), (2, false), (3, false), (4, true)];
        let decision = TreeBuilder::default().build(&mut data).unwrap();
        println!("{:?}", decision.to_string());
    }
}
