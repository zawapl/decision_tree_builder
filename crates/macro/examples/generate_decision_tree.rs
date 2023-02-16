use std::fs;
use std::fs::File;
use std::io::{Result, Write};

use decision_impl::TreeBuilder;
use decision_macro::TreeBuilderSupport;
use proc_macro2::TokenStream;

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
    write_to_file(token_stream, "Test".to_string()).unwrap();
}

pub fn write_to_file(token_stream: TokenStream, name: String) -> Result<()> {
    let formatted = token_stream.to_string();
    let output_folder = format!("test_data/generated");
    let output_filename = format!("{}/{}.rs", output_folder, name);
    fs::create_dir_all(output_folder)?;
    let mut file = File::create(output_filename)?;
    file.write_all(formatted.as_bytes())?;
    return Ok(());
}
