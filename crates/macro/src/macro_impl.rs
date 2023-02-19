use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::Attribute;
use syn::Data::Struct;

use crate::field_type::FieldType;
use crate::struct_field::StructField;

pub fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    let gain_calculator_struct_name = format_ident!("{struct_name}GainCalculator");

    if let Struct(data_struct) = &ast.data {
        let fields = StructField::from_fields(&data_struct.fields);

        let mut grc_fields = quote!(__results: HashMap<R, usize>);
        let mut grc_fields_init = quote!(__results: HashMap::new());
        let mut grc_add_entry = quote!( *self.__results.entry(*res).or_insert(0) += 1; );

        let mut grc_to_node_calculations = quote!(
            let gain_ratio_calculator = GainRatioCalculator::from_results(&self.__results);

            let mut best_gain = 0.0;
            let mut best_field = usize::MAX;
            let mut best_branch_size = usize::MAX;
        );

        let mut grc_match_clause = quote!(
            _ => {
                let most_common = *self.__results.iter().max_by(|a, b| (a.1).cmp(b.1)).map(|(k, _v)| k).unwrap();
                return Either::Left(most_common);
            }
        );

        for field in fields {
            let StructField { i, struct_field, named_field, field_type } = field;
            let field_type_ident = field_type.get_ident();
            let gain_ratio_calculator_function = field_type.gain_ratio_calculator_function();

            grc_fields = quote!(
                #grc_fields,
                #named_field: HashMap<#field_type_ident, HashMap<R, usize>>
            );

            grc_fields_init = quote!(
                #grc_fields_init,
                #named_field: HashMap::new()
            );

            grc_add_entry = quote!(
                #grc_add_entry
                *self.#named_field.entry(entry.#struct_field).or_insert(HashMap::new()).entry(*res).or_insert(0) += 1;
            );

            grc_to_node_calculations = quote!(
                #grc_to_node_calculations
                let #named_field = gain_ratio_calculator.#gain_ratio_calculator_function(&self.#named_field);

                if (#named_field.0 > best_gain) || (#named_field.0 == best_gain && #named_field.2 < best_branch_size){
                    best_gain = #named_field.0;
                    best_field = #i;
                    best_branch_size = #named_field.2;
                }
            );

            let ident_string = struct_field.to_string();
            if let FieldType::LiteralOrd(indent) = field_type {
                let literal_func = format_ident!("{}_unsuffixed", indent);
                grc_match_clause = quote!(
                    #i => {
                        let literal = Literal::#literal_func(#named_field.1);
                        let condition = utils::to_tokens_ord(literal, Ident::new(#ident_string, Span::call_site()));
                        let split = utils::split_data(data, |t| t.0.#struct_field < #named_field.1);
                        return Either::Right((condition, split));
                    },
                    #grc_match_clause
                );
            } else {
                grc_match_clause = quote!(
                    #i => {
                        let condition = utils::to_tokens_eq(#named_field.1, Ident::new(#ident_string, Span::call_site()));
                        let split = utils::split_data(data, |t| t.0.#struct_field == #named_field.1);
                        return Either::Right((condition, split));
                    },
                    #grc_match_clause
                );
            }
        }


        let gen = quote! {
            pub mod tree_builder_support {
                use decision_tree_builder_impl::*;
                use super::#struct_name;
                use std::collections::HashMap;
                use proc_macro2::{Ident, Literal, Span, TokenStream};
                use either::Either;
                use std::hash::Hash;

                impl<R: Copy + Eq + Hash> TreeBuilderSupport<R> for #struct_name {
                    type GainCalculator<T> = #gain_calculator_struct_name<R>;
                }

                pub struct #gain_calculator_struct_name<R>{
                    #grc_fields
                }

                impl<R: Copy + Eq + Hash> GainCalculator<#struct_name, R> for #gain_calculator_struct_name<R> {

                    fn new() -> Self {
                        return #gain_calculator_struct_name{#grc_fields_init};
                    }

                    fn add_entry(&mut self, (entry, res): &(#struct_name, R)) {
                        #grc_add_entry
                    }

                    fn to_node(self, data: &mut [(#struct_name, R)]) -> Either<R, (TokenStream, usize)> {
                        #grc_to_node_calculations
                        match best_field {
                            #grc_match_clause
                        }
                    }

                }
            }
        };

        return gen.into();
    }

    panic!("Not a struct");
}
