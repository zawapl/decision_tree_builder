use decision_tree_builder::TreeBuilderSupport;
use decision_tree_builder_impl::TreeBuilder;

#[derive(TreeBuilderSupport)]
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
    let token_stream = TreeBuilder::default().build(&mut test_data);
    let generated_ast = syn::parse2(token_stream).unwrap();
    let formatted = prettyplease::unparse(&generated_ast);
    println!("{}", formatted);
}
