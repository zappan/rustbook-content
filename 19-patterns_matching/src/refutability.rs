fn refutable_pattern() {
  // ## Attempting to use a refutable pattern with let will fail
  // let Some(x) = some_option_value;

  // ## Avoiding the refutability error by using if let
  // ## and a block with refutable patterns instead of let
  if let Some(x) = Some(33) {
    println!("This will print {x}");
  }
  if let Some(x) = None::<i32> {
    println!("This will NOT print {x}");
  }

  // ## If we give if let an irrefutable pattern (a pattern
  // ## that will always match), the compiler will give a warning.
  if let x = 5 {
    println!("{x}");
  };
}

fn irrefutable_pattern() {}

pub fn run() {
  refutable_pattern();
  irrefutable_pattern();
}
