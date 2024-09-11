use std::fmt::Display;

struct Point<T> {
  x: T,
  y: T,
}

struct MixedPoint<T, U> {
  x: T,
  y: U,
}

impl<T: Display> Point<T> {
  fn print(&self) {
    println!("The point coordinates are: ({}, {})", self.x, self.y);
  }
}

impl Point<f32> {
  fn print_float(&self) {
    println!("The point coordinates are: ({:.4}, {:.4})", self.x, self.y);
  }
}

impl<T: Display, U: Display> MixedPoint<T, U> {
  fn print(&self) {
    println!("The mixed-point coordinates are: ({}, {})", self.x, self.y);
  }
}

impl<T: Clone, U> MixedPoint<T, U> {
  fn mixup<V, W: Clone>(&self, other: &MixedPoint<V, W>) -> MixedPoint<T, W> {
    MixedPoint {
      x: self.x.clone(),
      y: other.y.clone(),
    }
  }
}

pub fn main() {
  let float_point = Point { x: 1.3, y: 2.2 };
  let int_point = Point { x: 2, y: 5 };
  float_point.print_float();
  float_point.print();
  int_point.print();
  println!(
    "Float point is: ({}, {}); while int point is: ({}, {})",
    float_point.x, float_point.y, int_point.x, int_point.y
  );

  let mixed_point = MixedPoint { x: 1.3, y: 3 };
  mixed_point.print();
  println!("Mixed point is ({}, {})", mixed_point.x, mixed_point.y);

  let nonmixed_point = MixedPoint { x: 1.3, y: 3.3 };
  nonmixed_point.print();
  println!(
    "Mixed point with non-mixed values is ({}, {})",
    nonmixed_point.x, nonmixed_point.y
  );

  let string_char_point = MixedPoint { x: "World", y: 'c' };
  let mixup_point = mixed_point.mixup(&string_char_point);
  println!("The MixUp point is ({}, {})", mixup_point.x, mixup_point.y);
  mixup_point.print();
  string_char_point.print();
  mixed_point.print();
}
