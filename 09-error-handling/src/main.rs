mod non_recoverable;
mod recoverable;

use std::error::Error as StdError;

fn main() -> Result<(), Box<dyn StdError>> {
  println!("Non-recoverable errors:");
  println!("-----------------------");
  non_recoverable::main();

  println!();
  println!("Recoverable errors:");
  println!("-------------------");
  recoverable::main()?;

  Ok(())
}
