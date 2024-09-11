// Using the Newtype Pattern to Implement External Traits on External Types
// ----------------------------------------------------------------------------------------------
// The orphan rule states we’re only allowed to implement a trait on a type if either the trait
// or the type are local to our crate. It’s possible to get around this restriction using the
// newtype pattern, which involves creating a new type in a tuple struct around the type we want
// to implement a trait for. Then we can implement the trait on the wrapper.
// ----------------------------------------------------------------------------------------------
// The downside of using this technique is that Wrapper is a new type, so it doesn’t have the
// methods of the value it’s holding.
// -- We would have to implement all the methods of Vec<T> directly on Wrapper such that the
//    methods delegate to self.0, which would allow us to treat Wrapper exactly like a Vec<T>.
// -- If we wanted the new type to have every method the inner type has, implementing the Deref
//    trait on the Wrapper to return the inner type would be a solution.
// -- If we don’t want the Wrapper type to have all the methods of the inner type (to restrict the
//    Wrapper type’s behavior) we would have to implement just the methods we do want manually.

use std::fmt;

// we want to implement Display on Vec<T>, which the orphan rule prevents us from doing directly

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let output = self
      .0
      .iter()
      .map(|i| i.to_uppercase())
      .collect::<Vec<_>>()
      .join(" :: ");

    write!(f, "[{}]", output)
  }
}

pub fn run() {
  println!("\nWRAPPERS:");
  let w = Wrapper(vec![
    String::from("hello world"),
    String::from("goodnight moon"),
  ]);

  println!("w = {}", w);
}
