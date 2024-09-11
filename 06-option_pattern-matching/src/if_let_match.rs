#[derive(Debug)]
enum Coin {
  _Penny,
  _Nickel,
  _Dime,
  _Quarter(USState),
}

#[derive(Debug)]
enum USState {
  _Alabama,
  _Alaska,
  // ----cut----
}

// The if let syntax lets you combine if and let into a less verbose way
// to handle values that match one pattern while ignoring the rest. Consider
// the program in Listing 6-6 that matches on an Option<u8> value in the config_max
// variable but only wants to execute code if the value is the Some variant.
pub fn if_let() {
  let config_max = Some(3u8);

  // the standard, verbose way (of executing some code if config value exists)
  match config_max {
    Some(max) => println!("The max is configured to be {}", max),
    _ => (), // to satisfy match expression, we need to add this, in this situation empty boilerplate code
  }

  // THE MORE CONCISE CODE BEHAVING IN THE SAME WAY:
  // The syntax if let takes a pattern and an expression separated by an equal sign.
  // It works the same way as a match, where the expression is given to the match and
  // the pattern is its first arm. In this case, the pattern is Some(max), and the max
  // binds to the value inside the Some. We can then use max in the body of the if let
  // block in the same way we used max in the corresponding match arm. The code in the
  // if let block isn’t run if the value doesn’t match the pattern.
  // ---------------------------------------------------------------------------------
  // However, you lose the exhaustive checking that match enforces. Choosing between
  // match and if let depends on what you’re doing in your particular situation and
  // whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.
  // ---------------------------------------------------------------------------------
  // In other words, you can think of if let as syntax sugar for a match that runs code
  // when the value matches one pattern and then ignores all other values.
  if let Some(max) = config_max {
    println!("The max is configured to be {}", max);
  }

  // WE CAN INCLUDE AN ELSE WITH AN IF LET:
  // using if-let-else
  let coin = Coin::_Nickel;
  let mut count = 0;

  if let Coin::_Quarter(ref state) = coin {
    println!("State quarter from {:?}", state);
  } else {
    count += 1;
  }

  println!("You have {:?} coin(s)", count);

  // same thing using match
  let coin = Coin::_Nickel;
  let mut count = 0;

  match coin {
    Coin::_Quarter(ref state) => println!("State quarter from {:?}", state),
    _ => count += 1,
  }

  println!("You have {:?} coin(s)", count);

  // PRINT THE SECOND FIELD OF Range IF LOC IS A Range
  #[derive(Debug)]
  enum Location {
    Point(u32),
    Range(u32, u32),
  }

  fn print_range_max(loc: &Location) {
    if let Location::Range(_, b) = *loc {
      println!("The second field of {:?} is {}", loc, b);
    }
  }

  print_range_max(&Location::Point(4));
  print_range_max(&Location::Range(6, 32));

  // RETURN THE FIRST FIELD OF Range OR THE ONLY FIELD OF Point
  fn get_start(loc: &Location) -> &u32 {
    match loc {
      Location::Point(a) => a,
      Location::Range(a, _) => a,
    }
  }

  let point = Location::Point(4);
  let range = Location::Range(6, 32);
  println!("The first field of {:?} is {}", point, get_start(&point));
  println!("The first field of {:?} is {}", range, get_start(&range));
}
