# decision_tree_builder

The package introduces a `TreeBuilderSupport` macro that allows the marked struct to be used by the `TreeBuilder`.
This allows for the builder to generate a token stream containing implementation of a decision tree created from the provided data using the C4.5 algorithm.

Example use (taken from [generate_decision_tree](crates/macro/examples/generate_decision_tree.rs) example):
```rust
#[derive(TreeBuilderSupport)]
#[TreeBuilderResultType(bool)]
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
```

Running above will output
```rust
pub fn decide(val: &generate_decision_tree::TestData) -> bool {
    if val.a < 1 {
        if val.b < 1 { false } else { true }
    } else {
        if val.b < 1 { true } else { false }
    }
}
```