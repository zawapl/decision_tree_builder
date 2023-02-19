use decision_tree_builder_impl::TreeBuilder;

pub struct TestData {
    a: usize,
    b: usize,
    c: bool,
    d: bool,
}

fn main() {
    let mut test_data = [
        (TestData { a: 0, b: 0, c: false, d: true }, false),
        (TestData { a: 0, b: 1, c: false, d: true }, true),
        (TestData { a: 1, b: 0, c: false, d: true }, true),
        (TestData { a: 1, b: 1, c: false, d: true }, false),
    ];
    let token_stream = TreeBuilder::default().build(&mut test_data).unwrap();
    let generated_ast = syn::parse2(token_stream).unwrap();
    let formatted = prettyplease::unparse(&generated_ast);
    println!("{}", formatted);
}

impl decision_tree_builder_impl::BranchBuilder for TestData {
    type Decision = __TestDataDecision;
    fn find_best_decision<R: Copy + Eq + std::hash::Hash, F, D>(entropy: f64, data: &mut [(D, R)], extract: F) -> Self::Decision
    where F: Fn(&D) -> &Self {
        use decision_tree_builder_impl::Decision;
        let decisions = [
            __TestDataDecision::a(decision_tree_builder_impl::BranchBuilder::find_best_decision(entropy, data, |d| &extract(d).a)),
            __TestDataDecision::b(decision_tree_builder_impl::BranchBuilder::find_best_decision(entropy, data, |d| &extract(d).b)),
            __TestDataDecision::c(decision_tree_builder_impl::BranchBuilder::find_best_decision(entropy, data, |d| &extract(d).c)),
            __TestDataDecision::d(decision_tree_builder_impl::BranchBuilder::find_best_decision(entropy, data, |d| &extract(d).d)),
        ];
        return decisions.into_iter().max_by(|a, b| a.to_decision_eval().cmp(b.to_decision_eval())).unwrap();
    }
    fn split_data<F, D, R>(data: &mut [(D, R)], extract: F, decision: &Self::Decision) -> usize
    where F: Fn(&D) -> &Self {
        return match decision {
            __TestDataDecision::a(inner) => decision_tree_builder_impl::BranchBuilder::split_data(data, |d| &extract(d).a, inner),
            __TestDataDecision::b(inner) => decision_tree_builder_impl::BranchBuilder::split_data(data, |d| &extract(d).b, inner),
            __TestDataDecision::c(inner) => decision_tree_builder_impl::BranchBuilder::split_data(data, |d| &extract(d).c, inner),
            __TestDataDecision::d(inner) => decision_tree_builder_impl::BranchBuilder::split_data(data, |d| &extract(d).d, inner),
        };
    }
}
pub enum __TestDataDecision {
    a(<usize as decision_tree_builder_impl::BranchBuilder>::Decision),
    b(<usize as decision_tree_builder_impl::BranchBuilder>::Decision),
    c(<bool as decision_tree_builder_impl::BranchBuilder>::Decision),
    d(<bool as decision_tree_builder_impl::BranchBuilder>::Decision),
}
impl decision_tree_builder_impl::Decision for __TestDataDecision {
    fn to_decision_eval(&self) -> &decision_tree_builder_impl::DecisionEval {
        return match self {
            __TestDataDecision::a(inner) => inner.to_decision_eval(),
            __TestDataDecision::b(inner) => inner.to_decision_eval(),
            __TestDataDecision::c(inner) => inner.to_decision_eval(),
            __TestDataDecision::d(inner) => inner.to_decision_eval(),
        };
    }
    fn to_condition(&self, var: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        use syn::__private::TokenStreamExt;
        let mut result = proc_macro2::TokenStream::new();
        result.append_all(var);
        result.append(proc_macro2::Punct::new('.', proc_macro2::Spacing::Alone));
        return match self {
            __TestDataDecision::a(inner) => {
                result.append(proc_macro2::Ident::new("a", proc_macro2::Span::call_site()));
                inner.to_condition(result)
            }
            __TestDataDecision::b(inner) => {
                result.append(proc_macro2::Ident::new("b", proc_macro2::Span::call_site()));
                inner.to_condition(result)
            }
            __TestDataDecision::c(inner) => {
                result.append(proc_macro2::Ident::new("c", proc_macro2::Span::call_site()));
                inner.to_condition(result)
            }
            __TestDataDecision::d(inner) => {
                result.append(proc_macro2::Ident::new("d", proc_macro2::Span::call_site()));
                inner.to_condition(result)
            }
        };
    }
}
