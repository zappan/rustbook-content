// If in your project you need to make the resulting binary as small as possible,
// you can switch from unwinding to aborting
// see: https://rust-book.cs.brown.edu/ch09-01-unrecoverable-errors-with-panic.html#unwinding-the-stack-or-aborting-in-response-to-a-panic

// run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// In order to get backtraces, debug symbols must be enabled. Debug symbols are enabled
// by default when using cargo build or cargo run without the --release flag, as we have here.

pub fn main() {
  // ## direct call to panic!
  // panic!("crash and burn!");

  // ## our code causing panic! through using someone else's code (std lib in this case)
  // let v = vec![1, 2, 3];
  // v[99]; // this would be buffer overread in C; Rust panics and disallows access

  println!("<Uncomment some of the above lines to cause panic!>")
}
