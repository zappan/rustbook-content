pub mod garden;

use garden::vegetables::Asparagus;

fn main() {
  let asparagus = Asparagus {};
  println!("I'm growing {:?}", asparagus);
}
