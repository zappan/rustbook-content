// As we did with let, we could match a tuple in a function’s arguments to the pattern,
// where Rust splits the values in a tuple as we pass it to a function.
fn print_coordinates(&(x, y): &(i32, i32)) {
  println!("Current location: ({x}, {y})");
}

pub fn run() {
  let point = (3, 5);
  print_coordinates(&point);
}
