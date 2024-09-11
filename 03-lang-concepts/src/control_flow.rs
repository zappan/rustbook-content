use crate::util::console;

fn if_statement() {
  console::section("'if' Expressions:");
  let num = 4;

  if num < 5 {
    println!("condition was true");
  } else {
    println!("condition was false");
  }

  if num % 4 == 0 {
    println!("num is divisible by 4");
  } else if num % 3 == 0 {
    println!("num is divisible by 3");
  } else if num % 2 == 0 {
    println!("num is divisible by 2");
  } else {
    println!("num is not divisible by 4, 3, or 2");
  }

  // --------------------------------------------------------------------------
  // Using too many else if expressions can clutter your code, so if you have
  // more than one, you might want to refactor your code. Chapter 6 describes
  // a powerful Rust branching construct called match for these cases.
  // --------------------------------------------------------------------------

  // Because if is an EXPRESSION, we can use it on the right side of a let statement to assign the outcome to a variable,
  let condition = true;
  let number = if condition { 5 } else { 9 }; // needs a semicolon at the end due to let variable assignment
  println!("The value of 'number' is {number}");
}

fn loops() {
  console::section("Loops:");

  // ### AN INFINITE LOOP - we won't be running that one :)
  // loop {
  //   println!("again!");
  // }
  println!("We won't run an infinite loop, but it's here...");

  // ### RETURNING VALUES FROM LOOP
  let mut counter = 0;

  let result = loop {
    counter += 1;
    if counter == 10 {
      break counter * 2; // you can add the value you want returned after the break expression
    }
  }; // needs a semicolon at the end due to let variable assignment

  println!("The result of a loop is {result}");
  console::spacer();

  // ### LOOP LABELS to Disambiguate Between Multiple Loops
  let mut count = 0;

  // NOTE the label only has a starting tick
  'counting_up: loop {
    println!("count = {count}");

    let mut remaining = 10; // shadowed in each iteration
    loop {
      println!("remaining = {remaining}");
      if remaining == 9 {
        break;
      }
      if count == 2 {
        break 'counting_up; // NOTE the label only has a starting tick
      }
      remaining -= 1;
    }

    count += 1;
  }
  println!("End count = {count}");
  console::spacer();

  // ## WHILE LOOPS
  let mut number = 3;
  while number != 0 {
    println!("{number}");
    number -= 1;
  }
  println!("LIFTOFF!!!");
  console::spacer();

  // ### FOR..IN LOOP for iterating over a collection/array
  // The safety and conciseness of for loops make them the most commonly used loop construct in Rust
  let a = [10, 20, 30, 40, 50];
  for el in a {
    println!("The value is: {el}");
  }
  console::spacer();

  // The way to use a for loop for the above number counting to liftoff would be to use a range
  for num in { 1..4 }.rev() {
    println!("{num}");
  }
  println!("LIFTOFF... WITH A FOR..IN LOOP!!!");
}

pub fn control_flow() {
  console::section("CONTROL FLOW:");

  if_statement();
  loops();
}
