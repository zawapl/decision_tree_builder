use std::default::Default;

use decision_tree_builder::BranchBuilder;
use decision_tree_builder_impl::TreeBuilder;

#[derive(BranchBuilder)]
pub struct TestData {
    a: f32,
    b: f64,
}

fn main() {
    let mut test_data = [
        (TestData { a: 1.0, b: 1.0 }, true),
        (TestData { a: 2.0, b: 2.0 }, true),
        (TestData { a: 2.0, b: 2.0 }, false),
        (TestData { a: 3.0, b: 3.0 }, false),
    ];
    let token_stream = TreeBuilder { show_conflicted_leaves: true, ..Default::default() }
        .build(&mut test_data)
        .unwrap();
    let generated_ast = syn::parse2(token_stream).unwrap();
    let formatted = prettyplease::unparse(&generated_ast);
    println!("{formatted}");
}
