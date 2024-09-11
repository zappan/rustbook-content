// ---
// https://rust-book.cs.brown.edu/ch15-04-rc.html
// ---
// There are cases when a single value might have multiple owners. For example, in
// graph data structures, multiple edges might point to the same node, and that node
// is conceptually owned by all of the edges that point to it. A node shouldn’t be
// cleaned up unless it doesn’t have any edges pointing to it and so has no owners.
// ---
// You have to enable multiple ownership explicitly by using the Rust type Rc<T>, which is an
// abbreviation for reference counting. The Rc<T> type keeps track of the number of references
// to a value to determine whether or not the value is still in use. If there are zero references
// to a value, the value can be cleaned up without any references becoming invalid.
// ---
// We use the Rc<T> type when we want to allocate some data on the heap for multiple parts of our
// program to read and we can’t determine at compile time which part will finish using the data last.
// ---
// Note that Rc<T> is only for use in single-threaded scenarios

// We’ll create list a that contains 5 and then 10. Then we’ll make two more lists:
// b that starts with 3 and c that starts with 4. Both b and c lists will then
// continue on to the first a list containing 5 and 10. In other words, both lists
// will share the first list containing 5 and 10.
use std::rc::Rc;
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
  Cons(i32, Rc<List>),
  Nil,
}

impl Drop for List {
  fn drop(&mut self) {
    println!("> Dropping {:?}", self);
  }
}

fn shared_cons_list() {
  let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
  println!("List {:?} created.", a);
  println!("  % Reference-count of `a` is now {}", Rc::strong_count(&a));

  // Note the immutable reference on Rc::clone() -- allows MUTLIPLE BORROWS
  let b = Cons(3, Rc::clone(&a));
  println!("List {:?} created.", b);
  println!("  % Reference-count of `a` is now {}", Rc::strong_count(&a));

  {
    // Note the immutable reference on Rc::clone() -- allows MUTLIPLE BORROWS
    let c = Cons(4, Rc::clone(&a));
    println!("List {:?} created.", c);
    println!("  % Reference-count of `a` is now {}", Rc::strong_count(&a));
  }

  println!("Finished.");
  println!("  % Reference-count of `a` is now {}", Rc::strong_count(&a));
}

pub fn run() {
  shared_cons_list();
}
