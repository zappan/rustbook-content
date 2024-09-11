// We can’t yet provide the hello_macro function with default implementation that will print
// the name of the type the trait is implemented on: Rust doesn’t have reflection capabilities,
// so it can’t look up the type’s name at runtime. We need a macro to generate code at compile time.
// --------------------
// The next step is to define the procedural macro. At the time of this writing, procedural macros
// need to be IN THEIR OWN CRATE. Eventually, this restriction might be lifted. The convention for
// structuring crates and macro crates is as follows: for a crate named foo, a custom derive
// procedural macro crate is called foo_derive. Our two crates are tightly related, so we create
// the procedural macro crate WITHIN THE DIRECTORY of our hello_macro crate.
// --------------------
pub trait HelloMacro {
  fn hello_macro();
}
