// ---
// RefCell<T> and the INTERIOR MUTABILITY PATTERN
// https://rust-book.cs.brown.edu/ch15-05-interior-mutability.html
// ---
// INTERIOR MUTABILITY is a design pattern in Rust that allows you to mutate data even
// when there are immutable references to that data. Normally, this action is disallowed
// by the borrowing rules; o mutate data, the pattern uses unsafe code inside a data
// structure to bend Rust’s usual rules that govern mutation and borrowing.
// ---
// We can use types that use the interior mutability pattern only when we can ensure that the
// borrowing rules will be followed at runtime, even though the compiler can’t guarantee that.
// The unsafe code involved is then wrapped in a safe API, and the outer type is still immutable.
// ---
// Let’s explore this concept by looking at the RefCell<T> type that follows the interior mutability pattern.
// ---
// Unlike Rc<T>, the RefCell<T> type represents single ownership over the data it holds.
//   * With references and Box<T>, the borrowing rules’ invariants are enforced at
//     compile time. With RefCell<T>, these invariants are enforced at runtime.
//   * With references, if you break these rules, you’ll get a compiler error. With
//     RefCell<T>, if you break these rules, your program will panic and exit.
// ---
// The RefCell<T> type is useful when you’re sure your code follows the borrowing rules but
// the compiler is unable to understand and guarantee that. Similar to Rc<T>, RefCell<T>
// is only for use in single-threaded scenarios and will give you a compile-time error
// if you try using it in a multithreaded context.
// ---

use std::cell::RefCell;
use std::rc::Rc;
use List::{Cons, Nil};

// ---
// A Use Case for Interior Mutability: Mock Objects
// Mock objects are specific types of test doubles that record what happens during
// a test so you can assert that the correct actions took place.
pub fn interior_mutability() {
  println!("See book chapter 15.5 for an example of RefCell<T>.");
}

// Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>
// ---
// A common way to use RefCell<T> is in combination with Rc<T>. Recall that Rc<T>
// lets you have multiple owners of some data, but it only gives immutable access
// to that data. If you have an Rc<T> that holds a RefCell<T>, you can get a value
// that can have multiple owners and that you can mutate!
#[derive(Debug)]
enum List {
  Cons(Rc<RefCell<i32>>, Rc<List>),
  Nil,
}

fn mutable_data_multiple_owners() {
  let shared_list_value = Rc::new(RefCell::new(5));
  println!(
    "Created `shared_list_value` with the value of {:?}",
    *shared_list_value.borrow()
  );
  println!(
    "  % Reference-count of `shared_list_value`: {}",
    Rc::strong_count(&shared_list_value)
  );

  let a = Rc::new(Cons(
    Rc::clone(&shared_list_value),
    Rc::new(Cons(Rc::new(RefCell::new(10)), Rc::new(Nil))),
  ));
  println!("Created list {:?}.", a);
  println!(
    "  % Reference-count of `shared_list_value`: {}",
    Rc::strong_count(&shared_list_value)
  );

  let b = Rc::new(Cons(Rc::new(RefCell::new(3)), Rc::clone(&a)));
  println!("Created list {:?}.", b);
  println!(
    "  % Reference-count of `shared_list_value`: {}",
    Rc::strong_count(&shared_list_value)
  );
  let c = Rc::new(Cons(Rc::new(RefCell::new(4)), Rc::clone(&a)));

  println!("Created list {:?}.", c);
  println!(
    "  % Reference-count of `shared_list_value`: {}",
    Rc::strong_count(&shared_list_value)
  );

  let incr = 10;
  print!(
    "Updating `shared_list_value` from {:?} to {}... ",
    shared_list_value.borrow(),
    (*shared_list_value.borrow() + incr)
  );
  *shared_list_value.borrow_mut() += incr;

  // std::thread::sleep(std::time::Duration::from_millis(900));
  println!("Done.");
  // std::thread::sleep(std::time::Duration::from_millis(600));

  println!("Updated list {:?}", a);
  println!("Updated list {:?}", b);
  println!("Updated list {:?}", c);
  println!(
    "  % Reference-count of `shared_list_value`: {}",
    Rc::strong_count(&shared_list_value)
  );

  // println!("{:?}", a.head().unwrap().borrow());
  // println!("{:?}", a.tail().unwrap().head().unwrap().borrow());
  // println!("{:?}", a.tail().unwrap().tail().unwrap());

  print!("Updating last item of the shared list `a` by {incr}... ");
  *a.tail().unwrap().head().unwrap().borrow_mut() += incr;

  // std::thread::sleep(std::time::Duration::from_millis(900));
  println!("Done.");
  // std::thread::sleep(std::time::Duration::from_millis(600));

  println!("Updated list {:?}", a);
  println!("Updated list {:?}", b);
  println!("Updated list {:?}", c);
}

impl List {
  // make it convenient to access the first item if we have a Cons variant
  fn head(&self) -> Option<&Rc<RefCell<i32>>> {
    match self {
      Cons(item, _) => Some(item),
      Nil => None,
    }
  }

  // make it convenient to access the second item if we have a Cons variant
  fn tail(&self) -> Option<&Rc<List>> {
    match self {
      Cons(_, item) => Some(item),
      Nil => None,
    }
  }
}

pub fn run() {
  interior_mutability();
  println!("-----");
  mutable_data_multiple_owners();
}
