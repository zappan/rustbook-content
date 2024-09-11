// Boxes allow you to store data on the heap rather than the stack.
// What remains on the stack is the pointer to the heap data.

// You’ll use them most often in these situations:
//   * When you have a type whose size can’t be known at compile time and you want
//     to use a value of that type in a context that requires an exact size
//   * When you have a large amount of data and you want to transfer ownership
//     but ensure the data won’t be copied when you do so
//   * When you want to own a value and you care only that it’s a type that implements
//     a particular trait rather than being of a specific type

// The cons list isn’t a commonly used data structure in Rust. Most of the time
// when you have a list of items in Rust, Vec<T> is a better choice to use.
#[derive(Debug)]
enum List {
  Cons(i32, Box<List>),
  Nil,
}

use List::{Cons, Nil};

fn cons_list() {
  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
  println!("{:?}", list);
}

pub fn run() {
  let b = Box::new(5); // Putting a single value on the heap isn’t very useful,
  println!("b = {b}"); //so you won’t use boxes by themselves in this way very often.

  cons_list();
}
