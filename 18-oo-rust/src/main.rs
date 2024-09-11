mod state_pattern_nonrust;
mod state_pattern_rust;
mod trait_objects;

// We’ve seen that even though Rust is capable of implementing object-oriented design patterns,
// other patterns, such as encoding state into the type system, are also available in Rust. These
// patterns have different trade-offs. Although you might be very familiar with object-oriented
// patterns, rethinking the problem to take advantage of Rust’s features can provide benefits,
// such as preventing some bugs at compile time. Object-oriented patterns won’t always be the best
// solution in Rust due to certain features, like ownership, that object-oriented languages don’t have.

fn main() {
  trait_objects::run();
  println!("\n=============================\n");
  state_pattern_nonrust::run();
  println!("\n=============================\n");
  state_pattern_rust::run();
}
