extern crate proc_macro;

use proc_macro::TokenStream;

mod macro_impl;
mod struct_field;

#[proc_macro_derive(BranchBuilder)]
pub fn my_macro_here_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    let token_stream = macro_impl::impl_hello_macro(&ast);


    // Save copy to output folder
    {
        use std::io::Write;

        let name = ast.ident.to_string();

        let generated_ast = syn::parse(token_stream.clone()).unwrap();
        let formatted = prettyplease::unparse(&generated_ast);
        let output_folder = format!(
            "{}/decision_trees/",
            std::env::var("OUT_DIR").unwrap_or(String::from("target"))
        );
        let output_filename = format!("{}/{}.rs", output_folder, name);
        std::fs::create_dir_all(output_folder);
        let mut file = std::fs::File::create(output_filename).unwrap();
        file.write_all(formatted.as_bytes());
    }

    return token_stream;
}
