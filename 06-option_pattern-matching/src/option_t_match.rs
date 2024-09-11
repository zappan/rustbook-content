fn enum_option_matches() {
  // This is the applied variation of Coin+USState combo (see enum_match module)
  // where It's always an Option of either Some(T) or None.
  // Allows accessing the value of an Option<T> through the code in Some arm when that case is hit
  fn plus_one(x: Option<i32>) -> Option<i32> {
    // Matches are exhaustive => the arms’ patterns must cover all possibilities,
    // otherwise the compiler will throw an error.
    match x {
      None => None,
      Some(num) => Some(num + 1),
    }
  }

  let some_number: Option<i32> = Some(5);
  println!("some_number is: {:?}", some_number);

  let another_number = plus_one(some_number);
  println!("another_number is: {:?}", another_number);

  let none = plus_one(None);
  println!("none is: {:?}", none);
}

fn enum_match_catch_all() {
  fn rolled_a_three() {
    println!("You rolled a three!!");
  }
  fn rolled_a_seven() {
    println!("You rolled a seven!!");
  }
  fn move_on(num: u8) {
    println!("Move on {num}, nothing to see here...");
  }
  fn roll_again() {
    println!("Roll again...");
  }

  let dice_roll = 9;

  // catch-all using any variable name, in this case we chose 'other'
  match dice_roll {
    3 => rolled_a_three(),
    7 => rolled_a_seven(),
    other => move_on(other),
  }

  // if we don't want to match & use the variable for a catch-all, there's a
  // placeholder '_' that can be used when we DON'T WANT TO USE the variable,
  // and having Rust not complaining about an unused variable:
  match dice_roll {
    3 => rolled_a_three(),
    7 => rolled_a_seven(),
    _ => roll_again(), // a special pattern that matches any value, and does NOT bind to that value
  }

  // finally, if we want nothing to happen, except on 3 and 7, we can express
  // that using a unit-value (the empty tuple type) at the catch-all/placeholder
  match dice_roll {
    3 => rolled_a_three(),
    7 => rolled_a_seven(),
    _ => (), // telling Rust we aren't going to use any other values, and we won't run any code in that case
  }
}

// fn enum_match_default_value() {
//   fn reset_to_zero(x: Option<i32>) -> Option<i32> {
//     match x {
//       _ => Some(0),
//     }
//   }

//   let some_number: Option<i32> = Some(5);
//   let zeroed = reset_to_zero(some_number);
//   println!("zeroed is: {:?}", zeroed);
// }

fn enum_match_on_a_reference() {
  let my_option_string: Option<String> = Some(String::from("Hello world"));

  // note match against an ampersand so that it doesn't lose the ownership...
  match &my_option_string {
    // Rust will “push down” the reference from the outer enum, &Option<String>,
    // to the inner field, &String. Therefore s has type &String,
    Some(s) => println!("{:?}", s),
    None => println!("None"),
  }
  // ...so that final println! can work
  println!("{:?}", my_option_string);
}

pub fn option_t_match() {
  enum_option_matches();
  enum_match_catch_all();
  // enum_match_default_value();
  enum_match_on_a_reference();
}
