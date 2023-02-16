use std::fs::File;
use std::io::{Result, Write};
use std::{env, fs};

use proc_macro::TokenStream;

pub fn write_to_file(token_stream: TokenStream, name: String) -> Result<()> {
    let generated_ast = syn::parse(token_stream).unwrap();
    let formatted = prettyplease::unparse(&generated_ast);
    let output_folder = format!("{}/decision_trees/", env::var("OUT_DIR").unwrap_or(String::from("target")));
    let output_filename = format!("{}/{}.rs", output_folder, name);
    fs::create_dir_all(output_folder)?;
    let mut file = File::create(output_filename)?;
    file.write_all(formatted.as_bytes())?;
    return Ok(());
}
