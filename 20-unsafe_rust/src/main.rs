mod external_code;
mod mutable_static;
mod raw_pointer;
mod safe_abstractions;
mod unsafe_function;
mod unsafe_trait;

// https://rust-book.cs.brown.edu/ch19-01-unsafe-rust.html
// -------------------------------------------------------------------------------------------
// Using unsafe to take one of the five actions (superpowers) isn’t wrong or even frowned upon.
// But it is trickier to get unsafe code correct because the compiler can’t help uphold memory
// safety. When you have a reason to use unsafe code, you can do so, and having the explicit
// unsafe annotation makes it easier to track down the source of problems when they occur.
// -------------------------------------------------------------------------------------------

fn main() {
  raw_pointer::run();
  unsafe_function::run();
  safe_abstractions::run();
  external_code::run();
  mutable_static::run();
  unsafe_trait::run();
}
