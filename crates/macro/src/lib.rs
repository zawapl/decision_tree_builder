mod macro_impl;
mod struct_field;
mod write_to_file;

extern crate proc_macro;

use std::fs::File;
use std::io::Write;
use std::{env, fs};

use proc_macro::TokenStream;
use quote::quote;
use syn::Data::Struct;

use crate::write_to_file::write_to_file;

#[proc_macro_derive(TreeBuilderSupport, attributes(TreeBuilderResultType))]
pub fn my_macro_here_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    let token_stream = macro_impl::impl_hello_macro(&ast);

    let name = ast.ident.to_string();

    // Save copy to output folder
    write_to_file(token_stream.clone(), name).unwrap();

    return token_stream;
}
