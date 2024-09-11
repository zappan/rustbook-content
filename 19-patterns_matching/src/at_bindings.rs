// -----------------------------------------------------------------------
// https://rust-book.cs.brown.edu/ch18-03-pattern-syntax.html#-bindings
// -----------------------------------------------------------------------

enum Message {
  Hello { id: i32 },
}

pub fn run() {
  let msg = Message::Hello { id: 5 };

  match msg {
    // Using @ lets us test a value and save it in a variable within one pattern.
    Message::Hello {
      id: id_variable @ 3..=7,
    } => println!("Found an id in range: {id_variable}"),
    // Here the code associated with the arm doesn’t have a variable that contains the actual value of the id field
    Message::Hello { id: 10..=12 } => {
      println!("Found an id in another range")
    }
    // We’ve specified a variable without a range, we do have the value available to use
    // in the arm’s code in a variable named id as we’ve used the struct field shorthand
    // syntax. But we haven’t applied any test to the value in the id field in this arm
    Message::Hello { id } => println!("Found some other id: {id}"),
  }
}
