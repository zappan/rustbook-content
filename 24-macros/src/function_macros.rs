// Function-like macros define macros that look like function calls.
//
// Function-like macros take a TokenStream parameter and their definition manipulates
// that TokenStream using Rust code as the other two types of procedural macros do.
//
// An example of a function-like macro is an sql! macro that might be called like so:
//
//    let sql = sql!(SELECT * FROM posts WHERE id=1);
//
// This macro would parse the SQL statement inside it and check that it’s syntactically
// correct, which is much more complex processing than a macro_rules! macro can do.
//
// The sql! macro would be defined like this:
//
//    #[proc_macro]
//    pub fn sql(input: TokenStream) -> TokenStream {
//
// This definition is similar to the custom derive macro’s signature: we receive the
// tokens that are inside the parentheses and return the code we wanted to generate.

pub fn run() {
  println!(
    "\nNo code samples for function-like macros, but check the comments inside the source file..."
  )
}
