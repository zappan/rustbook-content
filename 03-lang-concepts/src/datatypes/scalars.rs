use crate::util::console;

// ### INTEGER types: signed (i) and unsigned (u) 8/16/32/64/128/arch (arch: isize/usize)
// Number literals can use _ as a visual separator to make the number easier to read, such as 1_000
// integer types default to i32; isize or usize usually used when indexing some sort of collection.
fn integers() {
  console::section("Integers:");
  let guess: i32 = "42".parse().expect("Not a number!");
  println!("Value of an integer 'guess' is {guess}",);

  // ## INTERESTING !!!!
  // ## Integer overflow panics in dev mode, and overflows in release mode
  // ## https://rust-book.cs.brown.edu/ch03-02-data-types.html#integer-overflow
  // let x: u8 = 0;
  // println!("Value of u8 type -1 is {:?}", x - 1);
}

fn floats() {
  console::section("Floats:");

  let x = 2.0; // f64
  let y: f32 = 3.0; // f32
  println!("Float values of x[:64] and y:f32 are {x:.2} and {y:.5}, respectively");
}

// full list of operators in Rust: https://rust-book.cs.brown.edu/appendix-02-operators.html
fn numeric_operations() {
  console::section("Numeric operations:");

  let sum = 5 + 10;
  let difference = 95.5 - 4.3;
  // let product = 4 * 30.3; // not allowed mixing types
  let product = 4 * 30;
  let quotient = 56.7 / 32.2;
  let truncated_quotient = -5 / 3;
  let remainder = 43 % 5;

  println!("Sum is {sum}",);
  println!("Difference is {difference}",);
  println!("Product is {product}",);
  println!("Quotient of floats division is {quotient}",);
  println!("Truncated quotient of integers division is {truncated_quotient}",);
  println!("Remainder of a modulus operation is {remainder}",);
}

fn booleans() {
  console::section("Booleans:");
  let t = true;
  let f = false;
  println!("Values of t and f are {t} and {f}, respectively",);
}

// Rust’s CHAR type is FOUR BYTES in size and represents a UNICODE Scalar Value.
// --------------------------------------------------------------------------------------------
// That means it can represent a lot more than just ASCII. Accented letters, Chinese, Japanese,
// and Korean characters; emoji; and zero-width spaces are all valid char values in Rust.
// Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.
fn characters() {
  console::section("Characters:");
  let c = 'z';
  let z: char = 'Z'; // with explicit type annotation
  let heart_eyed_heart = '😻'; // unicode characters support
  println!("Values of char variables 'c' and 'z' are {c} and {z}, respectively");
  println!("Value of a unicode char variable 'heart_eyed_heart' is {heart_eyed_heart}");
}

pub fn scalars() {
  integers();
  floats();
  numeric_operations();
  booleans();
  characters();
}
