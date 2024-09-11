mod control_flow;
mod datatypes;
mod functions;
mod util;
mod variables;

use util::console;

fn main() {
  println!("Hello, world!");
  console::spacer();

  console::section("VARIABLES:");
  variables::variables();
  variables::constants();
  variables::shadowing();

  datatypes::datatypes();
  // functions::functions();
  // control_flow::control_flow();
  println!();
}
