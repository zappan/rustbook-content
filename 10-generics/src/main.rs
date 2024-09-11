mod enums;
mod largest;
mod structs;

fn main() {
  largest::main();
  println!("\n========================\n");
  structs::main();
  println!("\n========================\n");
  enums::main();
}
