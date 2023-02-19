use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;

use proc_macro2::{LexError, Literal, TokenStream};
use quote::quote;

use crate::{utils, BranchBuilder, Decision, ToFormattedTokens};

pub struct TreeBuilder {
    pub max_depth: usize,
    pub show_conflicted_leaves: bool,
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
        let show_conflicted_leaves = false;
        return TreeBuilder { max_depth, show_conflicted_leaves };
    }
}

impl TreeBuilder {
    pub fn build<T, R: ToFormattedTokens + Copy + Eq + Hash>(
        &self,
        data: &mut [(T, R)],
    ) -> Result<TokenStream, TreeBuilderError>
    where
        T: BranchBuilder,
    {
        let result_type = TokenStream::from_str(std::any::type_name::<R>())?;
        let input_type = TokenStream::from_str(std::any::type_name::<T>())?;
        let context = TreeBuilderContext::new();
        let inner = self.build_branch(&context, data)?;
        return Ok(quote!(pub fn decide(val: &#input_type) -> #result_type {
            return #inner;
        }));
    }

    fn build_branch<T, R: ToFormattedTokens + Copy + Eq + Hash>(
        &self,
        context: &TreeBuilderContext,
        data: &mut [(T, R)],
    ) -> Result<TokenStream, TreeBuilderError>
    where
        T: BranchBuilder,
    {
        let counts = utils::to_counts(&data);

        // If there is only one possible result left in this branch
        // or if max depth has been reached
        if counts.len() < 2 || context.depth > self.max_depth {
            return Ok(self.get_most_common_result(&counts));
        }

        let entropy = utils::entropy(&counts);

        let decision = BranchBuilder::find_best_decision(entropy, &mut data[..], |v| v);

        let condition = decision.to_condition(TokenStream::from_str("val")?);

        let split = BranchBuilder::split_data(&mut data[..], |v| v, &decision);

        if split == 0 || split == data.len() {
            return Ok(self.get_most_common_result(&counts));
        }

        let next_context = context.next();
        let branch_a = self.build_branch(&next_context, &mut data[..split])?;
        let branch_b = self.build_branch(&next_context, &mut data[split..])?;

        return Ok(quote!(
            if #condition {
                #branch_a
            } else {
                #branch_b
            }
        ));
    }

    fn get_most_common_result<R: Eq + ToFormattedTokens>(
        &self,
        map: &HashMap<R, usize>,
    ) -> TokenStream {
        return if map.len() == 1 {
            return map.keys().next().unwrap().to_formatted_tokens();
        } else {
            let result = map.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0.to_formatted_tokens();
            let mut comment = TokenStream::new();
            if self.show_conflicted_leaves {
                for (r, c) in map {
                    let label = r.to_formatted_tokens();
                    let count = Literal::usize_unsuffixed(*c);
                    comment = quote!(#comment #[tree_builder_conflicted_leaf(#label = #count)])
                }
            }
            quote!(#comment #result)
        };
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use crate::tree_builder::TreeBuilder;

    #[test]
    fn test_bool() {
        let mut data = [(true, 1), (false, 2)];
        let decision = TreeBuilder::default().build(&mut data).unwrap();
        let expected = quote!(
            pub fn decide(val: &bool) -> i32 {
                return if val { 1 } else { 2 };
            }
        );
        assert_eq!(decision.to_string(), expected.to_string());
    }
}
