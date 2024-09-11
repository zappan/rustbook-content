// Let is also a pattern. To see the pattern matching aspect of let more clearly,
// consider Listing 18-4, which uses a pattern with let to destructure a tuple.
// We match a tuple against a pattern.
pub fn run() {
  let (x, y, z) = (1, 2, 3);
  println!("{x} {y} {z}");
}
