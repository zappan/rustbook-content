// ----------------------------------------------------------------------------------------------------
// Sometimes, you might write a trait definition that depends on another trait: for a type to implement
// the first trait, you want to require that type to also implement the second trait. You would do this
// so that your trait definition can make use of the associated items of the second trait. The trait
// your trait definition is relying on is called a supertrait of your trait.
// ----------------------------------------------------------------------------------------------------

use std::fmt;

// OutlinePrint trait will work only for types that also implement Display. We can
// do that in the trait definition by specifying OutlinePrint: Display.
trait OutlinePrint: fmt::Display {
  fn outline_print(&self) {
    let output = self.to_string();
    let len = output.len();
    println!("{}", "*".repeat(len + 4));
    println!("*{}*", " ".repeat(len + 2));
    println!("* {output} *");
    println!("*{}*", " ".repeat(len + 2));
    println!("{}", "*".repeat(len + 4));
  }
}

#[derive(Debug)]
struct Point {
  x: i32,
  y: i32,
}

// impl fmt::Display for Point {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     write!(f, "({}, {})", self.x, self.y)
//   }
// }

impl fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    write!(f, "({}, {})", self.x, self.y)
  }
}

impl OutlinePrint for Point {}

pub fn run() {
  let p = Point { x: 12, y: 22 };
  println!("\n{}\n", p);
  p.outline_print();
}
