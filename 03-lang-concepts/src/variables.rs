use crate::util::console;

// ### Variables
// In Rust, variables are immutable by default. However, you still have
// the option to make your variables mutable.
// -------------------------------------------------------------------------------------------
// This is one of many nudges Rust gives you to write your code in a way that takes
// advantage of the safety and easy concurrency that Rust offers. Let’s explore how and
// why Rust encourages you to favor immutability and why sometimes you might want to opt out.
// -------------------------------------------------------------------------------------------
pub fn variables() {
  console::section("Variables:");

  // immutable variable
  let x = 5;
  println!("The value of x is: {x}");
  // x = 6  ### will throw a compiler error 'cannot assign twice to immutable variable'

  // mutable variable
  let mut y = 6;
  println!("The value of mutable y is: {y}");

  y = 7;
  println!("The value of mutable y is now: {:?}", y); // looks like this is for allowing nicer formatting on long lines...
}

// ### Constants
// Like immutable variables, constants are values that are bound to a name and are  NOT allowed
// to change, but there are a few differences between constants and variables. First, you aren’t
// allowed to use mut with constants. Constants aren’t just immutable by default—they’re always immutable.
// -------------------------------------------------------------------------------------------
// Constants evaluation during compilation: https://doc.rust-lang.org/reference/const_eval.html
// "Only a subset of all expressions can be evaluated at compile-time."
// -------------------------------------------------------------------------------------------
// Constants, unlike variables, CAN BE USED IN THE GLOBAL SCOPE (while variables only inside functions?)
// "Constants are valid for the entire time a program runs, within the scope in which they were declared.
// This property makes constants useful for values in your application domain that multiple parts of the
// program might need to know about.
// -------------------------------------------------------------------------------------------
pub fn constants() {
  console::section("Constants:");
  const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
  println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
}

// ### Shadowing
// You can declare a new variable with the same name as a previous variable
// -- the interesting part is that you can do it within the same scope!! --
pub fn shadowing() {
  console::section("Shadowing:");
  let x = 5;

  let x = x + 1; // ## ALLOWED as it containes the let keywoard again (SHADOWING)

  {
    let x = x * 2; // ...this is a different scope, even though not a function, could be valid in other langs...
    println!("The value of x in the inner scope is: {x}");
  }

  println!("The value of x is: {x}",);

  // ## with shadowing we can change the type of the value but reuse the same name
  let x = "By the virtue of shadowing, I am now a string";
  println!("The value of x is: {x}");
}
