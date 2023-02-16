use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{format_ident, quote};
use syn::Attribute;
use syn::Data::Struct;

use crate::struct_field::StructField;

pub fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gain_calculator_type = format_ident!("{name}GainCalculator");
    let result_type = get_result_type(&ast.attrs);

    if let Struct(dataStruct) = &ast.data {
        let fields = StructField::from_fields(&dataStruct.fields);

        let mut gain_calculator_fields = quote!( __results: HashMap<#result_type, usize> );
        let mut gain_calculator_fields_init = quote!(__results: HashMap::new());
        let mut gain_calculator_add_entry = quote!( *self.__results.entry(*res).or_insert(0) += 1; );

        let mut gain_calculator_to_node_calculations = quote!(
            let entropy_calculator = GainRatioCalculator::from_results(&self.__results);

            let mut best_gain = 0.0;
            let mut best_field = usize::MAX;
            let mut best_branch_size = usize::MAX;
        );

        let mut gain_calculator_matching = quote!(
            _ => {
                let most_common = *self.__results.iter().max_by(|a, b| (a.1).cmp(b.1)).map(|(k, _v)| k).unwrap();
                return Either::Left(most_common);
            }
        );

        for field in fields {
            let gain_calculator_field = &field.gain_calculator_field;
            let field_type = &field.ty;
            let struct_field = &field.struct_field;
            let gain_ratio_calculator_call = &field.gain_ratio_calculator_call;
            let is_equals = field.is_equals;
            let i = &field.i;

            gain_calculator_fields = quote!(
                #gain_calculator_fields,
                #gain_calculator_field: HashMap<#field_type, HashMap<bool, usize>>
            );

            gain_calculator_fields_init = quote!(
                #gain_calculator_fields_init,
                #gain_calculator_field: HashMap::new()
            );

            gain_calculator_add_entry = quote!(
                #gain_calculator_add_entry
                *self.#gain_calculator_field.entry(entry.#struct_field).or_insert(HashMap::new()).entry(*res).or_insert(0) += 1;
            );

            gain_calculator_to_node_calculations = quote!(
                #gain_calculator_to_node_calculations
                let #gain_calculator_field = entropy_calculator.#gain_ratio_calculator_call(&self.#gain_calculator_field);

                if (#gain_calculator_field.0 > best_gain) || (#gain_calculator_field.0 == best_gain && #gain_calculator_field.2 < best_branch_size){
                    best_gain = #gain_calculator_field.0;
                    best_field = #i;
                    best_branch_size = #gain_calculator_field.2;
                }
            );

            let ident_string = struct_field.to_string();
            if is_equals {
                gain_calculator_matching = quote!(
                    #i => {
                        // let literal = Literal::#literal_func(#gain_calculator_field.1);
                        let condition = Decision::to_tokens_eq(#gain_calculator_field.1, Ident::new(#ident_string, Span::call_site()));
                        let split = split_data(data, |t| t.0.#struct_field == #gain_calculator_field.1);
                        return Either::Right((condition, split));
                    },
                    #gain_calculator_matching
                );
            } else {
                gain_calculator_matching = quote!(
                    #i => {
                        let condition = Decision::to_tokens_ord(#gain_calculator_field.1, Ident::new(#ident_string, Span::call_site()));
                        let split = split_data(data, |t| t.0.#struct_field < #gain_calculator_field.1);
                        return Either::Right((condition, split));
                    },
                    #gain_calculator_matching
                );
            }
        }


        let gen = quote! {
            pub mod tree_builder_support {
                use decision_impl::*;
                use either::Either;
                use super::#name;
                use std::collections::HashMap;
                use proc_macro2::{Ident, Span, TokenStream, Literal};

                impl decision_impl::TreeBuilderSupport for #name {
                    type GainCalculator = #gain_calculator_type;
                    type ResultType = #result_type;
                }

                pub struct #gain_calculator_type{
                    #gain_calculator_fields
                }

                impl decision_impl::GainCalculator<#name, #result_type> for #gain_calculator_type {

                    fn new() -> Self {
                        return #gain_calculator_type{#gain_calculator_fields_init};
                    }

                    fn add_entry(&mut self, (entry, res): &(#name, #result_type)) {
                        #gain_calculator_add_entry
                    }

                    fn to_node(self, data: &mut [(#name, #result_type)]) -> Either<#result_type, (TokenStream, usize)> {
                        #gain_calculator_to_node_calculations
                        match (best_field) {
                            #gain_calculator_matching
                        }
                    }

                }
            }
        };

        return gen.into();
    }

    panic!("Not a struct");
}

fn get_result_type(attrs: &Vec<Attribute>) -> Ident {
    for attr in attrs {
        if attr.path.get_ident().map(|ident| ident.eq(&format_ident!("TreeBuilderResultType"))).unwrap_or(false) {
            let inner_val = attr.parse_args().unwrap_or_else(|_| {
                panic!("Expected usage: #[TreeBuilderResultType(type)]");
            });
            return inner_val;
        }
    }

    panic!("Missing: #[TreeBuilderResultType(type)]")
}
