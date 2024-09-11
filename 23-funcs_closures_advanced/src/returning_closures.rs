// The following code tries to return a closure directly, but it won’t compile:
//
//   fn returns_closure() -> dyn Fn(i32) -> i32 {
//     |x| x + 1
//   }
//
// The error references the Sized trait again! Rust doesn’t know how much space it will need
// to store the closure. We saw a solution to this problem earlier. We can use a trait object:
// ------------------
// For more about trait objects, refer to the section “Using Trait Objects That Allow for Values of Different Types”
// https://rust-book.cs.brown.edu/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
// ------------------
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
  Box::new(|x| x + 1)
}

pub fn run() {
  let closure = returns_closure();
  let result = closure(4);
  println!("Closure results in {}", result);
}
