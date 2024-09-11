use add_one;
use add_two;

fn main() {
  let num = 10;
  println!(
    "Hello, world! {num} plus one is {}, while {num} plus two is {}",
    add_one::add_one(num),
    add_two::add_two(num)
  );
}
