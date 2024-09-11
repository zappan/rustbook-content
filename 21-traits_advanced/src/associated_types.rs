struct Counter {
  count: u32,
}

// One example of a trait with an associated type is the Iterator trait that the standard
// library provides. The associated type is named Item and stands in for the type of the
// values the type implementing the Iterator trait is iterating over. The type Item is a
// placeholder, and the next method’s definition shows that it will return values of type
// Option<Self::Item>.
pub trait Iterator {
  type Item;

  fn next(&mut self) -> Option<Self::Item>;
}

// Implementors of the Iterator trait will specify the concrete type for Item, and the next
// method will return an Option containing a value of that concrete type.
impl Iterator for Counter {
  //
  // implementation of the `Item` from the trait
  type Item = u32;

  // NOTE: this implementation doesn't make any real sense, it's just to showcase the syntax
  fn next(&mut self) -> Option<Self::Item> {
    Some(self.count + 1)
  }
}

pub fn run() {
  let mut c = Counter { count: 0 };

  let x = c.next();
  println!("{}", x.unwrap())
}
