use crate::util::console;

fn print_labeled_measurement(value: i32, unit_label: char) {
  println!("The measurement is: {value}{unit_label}");
}

// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value.
fn statements_and_expressions() {
  // function definitions are also statements
  let _y = 6; // statement

  // STATEMENTS do not return values. Therefore, you can’t assign a let statement
  // to another variable, as the following code tries to do; you’ll get an error:
  // let x = (let y = 6);
  // This is different from what happens in other languages, such as C and Ruby, where
  // the assignment returns the value of the assignment. In those languages, you can
  // write x = y = 6 and have both x and y have the value 6; that is not the case in Rust.

  // in the above statement let y = 6;, the 6 is an EXPRESSION that evaluates to the value 6.
  // Calling a function is an expression.
  // Calling a macro is an expression.
  // A new scope block created with curly brackets is an expression

  // ----------------------------------------------------------------------------------------

  // Note that the x + 1 line below doesn’t have a semicolon at the end, which is unlike
  // most of the lines you’ve seen so far. Expressions do not include ending semicolons.
  // If you add a semicolon to the end of an expression, you turn it into a statement,
  // and it will then not return a value.
  // Keep this in mind as you explore function return values and expressions next.
  let y = {
    let x = 3;
    x + 1
  };
  println!("The value of y is: {y}");
}

// We don’t name return values, but we must declare their type after an arrow (->).
// In Rust, the return value of the function is synonymous with the value of the
// final expression in the block of the body of a function.
fn five() -> i32 {
  5 // a lonely 5 with no semicolon because it’s an expression whose value we want to return
}

fn plus_one(x: i32) -> i32 {
  x + 1 // no semicolon because it’s an expression whose value we want to return
}

fn functions_with_return_values() {
  let x = five();
  println!("Value of x is {x}");
  println!("value of plus_one(five()) is {:?}", plus_one(five()));
}

// note the curly-brace block which is an expression (allowing statements, and returning a value)
// and a syntactic scope (for let-bindings inside)
fn functions_with_syntactic_scope() {
  println!(
    "{}",
    plus_one({
      let y = 6;
      y + 2
    })
  );
}

pub fn functions() {
  console::section("Functions:");

  print_labeled_measurement(5, 'h');
  statements_and_expressions();
  functions_with_return_values();
  functions_with_syntactic_scope();
}
