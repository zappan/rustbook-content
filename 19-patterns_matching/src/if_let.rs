// It’s also possible to mix and match if let, else if, and else if let expressions.
// Doing so gives us more flexibility than a match expression in which we can express
// only one value to compare with the patterns. Also, Rust doesn’t require that the
// conditions in a series of if let, else if, else if let arms relate to each other.
// -------------
// The downside of using if let expressions is that the compiler doesn’t check for exhaustiveness,
// whereas with match expressions it does. If we omitted the last else block and therefore
// missed handling some cases, the compiler would not alert us to the possible logic bug.
pub fn run() {
  let favorite_color: Option<&str> = None;
  let is_tuesday = false;
  let age: Result<u8, _> = "34".parse();

  if let Some(color) = favorite_color {
    println!("Using your favorite color, {color}, as the background");
  } else if is_tuesday {
    println!("Tuesday is green day!");
  } else if let Ok(age) = age {
    if age > 30 {
      println!("Using purple as the background color");
    } else {
      println!("Using orange as the background color");
    }
  } else {
    println!("Using blue as the background color");
  }
}
