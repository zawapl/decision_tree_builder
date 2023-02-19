use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::Data::Struct;
use syn::__private::TokenStream2;

use crate::struct_field::StructField;

pub fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    let decision_enum_name = format_ident!("__{struct_name}Decision");

    if let Struct(data_struct) = &ast.data {
        let fields = StructField::from_fields(&data_struct.fields);

        let mut find_best_decision = TokenStream2::new();
        let mut split_data_match = TokenStream2::new();
        let mut decision_enum_options = TokenStream2::new();
        let mut decision_enum_to_decision_eval_match = TokenStream2::new();
        let mut decision_enum_to_condition_match = TokenStream2::new();

        for field in fields {
            let named_field = field.named_field;
            let struct_field = field.struct_field;
            let field_type = field.field_type;
            let struct_field_string = struct_field.to_string();

            find_best_decision = quote!(
                #find_best_decision
                #decision_enum_name::#named_field(decision_tree_builder_impl::BranchBuilder::find_best_decision(entropy, data, |d| &extract(d).#struct_field)),
            );

            split_data_match = quote!(
                #split_data_match
                #decision_enum_name::#named_field(inner) => decision_tree_builder_impl::BranchBuilder::split_data(data, |d| &extract(d).#struct_field, inner),
            );

            decision_enum_options = quote!(
                #decision_enum_options
                #named_field(<#field_type as decision_tree_builder_impl::BranchBuilder>::Decision),
            );

            decision_enum_to_decision_eval_match = quote! (
                #decision_enum_to_decision_eval_match
                #decision_enum_name::#named_field(inner) => inner.to_decision_eval(),
            );

            decision_enum_to_condition_match = quote!(
                #decision_enum_to_condition_match
                #decision_enum_name::#named_field(inner) => {
                    result.append(proc_macro2::Ident::new(#struct_field_string, proc_macro2::Span::call_site()));
                    inner.to_condition(result)
                }
            );
        }


        let gen = quote! {
            impl decision_tree_builder_impl::BranchBuilder for #struct_name {
                type Decision = #decision_enum_name;

                fn find_best_decision<R: Copy + Eq + std::hash::Hash, F, D>(entropy: f64, data: &mut [(D, R)], extract: F) -> Self::Decision
                where F: Fn(&D) -> &Self {
                    use decision_tree_builder_impl::Decision;

                    let decisions= [
                        #find_best_decision
                    ];

                    return decisions.into_iter()
                        .max_by(|a, b| a.to_decision_eval().cmp(b.to_decision_eval()))
                        .unwrap();
                }

                fn split_data<F, D, R>(data: &mut [(D, R)], extract: F, decision: &Self::Decision) -> usize
                where F: Fn(&D) -> &Self {
                    return match decision {
                        #split_data_match
                    };
                }
            }

            pub enum #decision_enum_name {
                #decision_enum_options
            }

            impl decision_tree_builder_impl::Decision for #decision_enum_name {
                fn to_decision_eval(&self) -> &decision_tree_builder_impl::DecisionEval {
                    return match self {
                        #decision_enum_to_decision_eval_match
                    };
                }

                fn to_condition(&self, var: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
                    use syn::__private::TokenStreamExt;

                    let mut result = proc_macro2::TokenStream::new();
                    result.append_all(var);
                    result.append(proc_macro2::Punct::new('.', proc_macro2::Spacing::Alone));
                    return match self {
                        #decision_enum_to_condition_match
                    };
                }
            }

        };

        return gen.into();
    }

    panic!("Not a struct");
}
