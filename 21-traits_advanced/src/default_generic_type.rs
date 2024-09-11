use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
  x: i32,
  y: i32,
}

// # We can specify a default concrete type for the generic type. This eliminates the need for
// # implementors of the trait to specify a concrete type if the default type works. You specify
// # a default type when declaring a generic type with the <PlaceholderType=ConcreteType> syntax.
// --------------------------------------------------------------------------------------------
// # The default generic type in this code is within the Add trait. Here is its definition:
// # Note the Rhs=Self part: this syntax is called default type parameters. The Rhs (“right
// # hand side”) generic type parameter defines the type of the rhs parameter in the add method.
// # If we don’t specify a concrete type for Rhs when we implement the Add trait, the type of
// # Rhs will default to Self, which will be the type we’re implementing Add on.
// --------------------------------------------------------------------------------------------
//
// trait Add<Rhs=Self> {
//     type Output;
//
//     fn add(self, rhs: Rhs) -> Self::Output;
// }
//

// Notice that there is no generic definition on the trait implementation, default generic type is used
impl Add for Point {
  type Output = Point;

  fn add(self, other: Point) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

// If we want to add values in millimeters to values in meters and have the implementation of Add do the
// conversion correctly. We can implement Add for Millimeters with Meters as the Rhs
#[derive(Debug, Clone, Copy)]
struct Millimeters(u32);

#[derive(Debug, Clone, Copy)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
  type Output = Millimeters;

  fn add(self, other: Meters) -> Millimeters {
    Millimeters(self.0 + (other.0 * 1000))
  }
}

pub fn run() {
  let p1 = Point { x: 1, y: 0 };
  let p2 = Point { x: 2, y: 3 };
  let result = p1 + p2;
  println!(
    "Result of adding points {:?} and {:?} is {:?}",
    p1, p2, result
  );

  println!("----- Adding metres to millimetres -----");
  let mm = Millimeters(24);
  let m = Meters(2);
  let sum = mm + m;
  println!("Sum of {:?} and {:?} is {:?}", mm, m, sum);
}
