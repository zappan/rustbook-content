mod enum_vectors;
mod hashmaps;
mod strings;
mod util;
mod vectors;

use util::console;

// built-in types: ARRAY, TUPLE
// reference (heap; pointed-to) types: VECTOR, STRING, HASH-MAP
// -> which means the amount of data does not need to be known at
//    compile time and can grow or shrink as the program runs.
fn main() {
  vectors::vectors();
  console::spacer();

  enum_vectors::enum_vectors();
  console::spacer();

  strings::strings();
  console::spacer();

  hashmaps::hashmaps();
  console::spacer();
}
