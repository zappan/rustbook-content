#[derive(Debug)]
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(USState),
}

#[derive(Debug)]
enum USState {
  Alabama,
  Alaska,
  // ----cut----
}

fn value_in_cents(coin: Coin) -> u8 {
  // format: match <expression> -- pretety much a switch-case
  match coin {
    // all the following are called "MATCH ARMS" (resolving options) fomatted as: pattern => some_code
    Coin::Penny => 1,

    Coin::Nickel => {
      // match arm expression can have multiple lines of code, then falls between curly braces
      println!("Don't confuse a nickel for a dime");
      5
    } // comma here is optional when curlies used

    Coin::Dime => 10,

    // Another useful feature of match arms is that they can bind to the parts of the values
    // that match the pattern. This is how we can extract values out of enum variants.
    // THIS WILL BE USED TO EXTRACT TYPE <T> FROM Option<T>
    Coin::Quarter(us_state) => {
      println!("State quarter from: {:?}!", us_state);
      25
    }
  }
}

fn coins_values() {
  println!(
    "Value of a {:?} is {} cents",
    Coin::Penny,
    value_in_cents(Coin::Penny)
  );
  println!(
    "Value of a {:?} is {} cents",
    Coin::Nickel,
    value_in_cents(Coin::Nickel)
  );
  println!(
    "Value of a {:?} is {} cents",
    Coin::Dime,
    value_in_cents(Coin::Dime)
  );
  println!(
    "Value of a {:?} is {} cents",
    Coin::Quarter(USState::Alabama),
    value_in_cents(Coin::Quarter(USState::Alabama))
  );
  println!(
    "Value of a {:?} is {} cents",
    Coin::Quarter(USState::Alaska),
    value_in_cents(Coin::Quarter(USState::Alaska))
  );
}

pub fn enum_match() {
  coins_values();
}
