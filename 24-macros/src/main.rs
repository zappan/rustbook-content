// ----------------------------------------------------------------------------
// https://rust-book.cs.brown.edu/ch19-06-macros.html
// ----------------------------------------------------------------------------
// The term macro refers to a family of features in Rust: DECLARATIVE macros
// with macro_rules! and three kinds of PROCEDURAL macros:
// * Custom #[derive] macros that specify code added with the derive attribute used on STRUCTS and ENUMS
// * Attribute-like macros that define custom attributes usable on ANY item
// * Function-like macros that look like function calls but operate on the tokens specified as their argument

// Declarative Macros with macro_rules! -- for General Metaprogramming
// Procedural Macros -- for Generating Code from Attributes

mod attribute_macros;
mod derive_macros;
mod function_macros;

fn main() {
  derive_macros::run();
  attribute_macros::run();
  function_macros::run();
}
