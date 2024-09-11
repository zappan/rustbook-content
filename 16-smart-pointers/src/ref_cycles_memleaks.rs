// By using Rc<T> and RefCell<T>: it’s possible to create references where items
// refer to each other in a cycle. This creates memory leaks because the reference
// count of each item in the cycle will never reach 0, and the values will never be dropped.

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use List::{Cons, Nil};

#[derive(Debug)]
enum List {
  Cons(i32, RefCell<Rc<List>>), // here, we can modify the List value a Cons variant is pointing to
  Nil,
}

impl List {
  // make it convenient to access the second item if we have a Cons variant
  fn tail(&self) -> Option<&RefCell<Rc<List>>> {
    match self {
      Cons(_, item) => Some(item),
      Nil => None,
    }
  }
}

fn ref_cycle_memleak() {
  let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
  println!("`a` initial rc count = {}", Rc::strong_count(&a));
  println!("`a` next item = {:?}", a.tail());

  let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
  println!("`b` initial rc count = {}", Rc::strong_count(&b));
  println!("`b` (pointing to `a`) next item = {:?}", b.tail());
  println!("`a` rc count after b creation = {}", Rc::strong_count(&a));

  if let Some(list_link) = a.tail() {
    println!("--------");
    println!("`a` last item before linking `a` to `b`: {:?}", list_link);
    *list_link.borrow_mut() = Rc::clone(&b); // link a to b

    println!("`b` rc count after changing `a` = {}", Rc::strong_count(&b));
    println!("`a` rc count after changing `a` = {}", Rc::strong_count(&a));

    println!("!!!!! We'd get a stack overflow runtime error if we now debug-print `a` or `b` !!!!!")
    // ## Uncomment one of the the next two lines to see that we have a cycle;
    // ## it will overflow the stack
    // println!("Link after linking `a` to `b`: {:?}", list_link);
  }

  // ------------------------------------------------------------------------------------------
  // At the end of function, Rust drops the variable b, which decreases the reference count
  // of the b Rc<List> instance from 2 to 1. The memory that Rc<List> has on the heap won’t
  // be dropped at this point, because its reference count is 1, not 0. Then Rust drops a,
  // which decreases the reference count of the a Rc<List> instance from 2 to 1 as well.
  // This instance’s memory can’t be dropped either, because the other Rc<List> instance
  // still refers to it. The memory allocated to the list will remain uncollected forever.
  println!("-----------------------------------------------------------------------------------");
  println!(
    "!!!!! We'll now leave, having memory allocated to the lists remain uncollected forever, \
    due to a cyclic reference. !!!!!"
  );
  println!("-----------------------------------------------------------------------------------");
  println!();
}

// Strong references are how you can share ownership of an Rc<T> instance. Weak references
// don’t express an ownership relationship, and their count doesn’t affect when an Rc<T>
// instance is cleaned up. They won’t cause a reference cycle because any cycle involving
// some weak references will be broken once the strong reference count of values involved is 0.
// ------------------------------------------------------------------------------------------------
// Preventing Reference Cycles: Turning an Rc<T> into a Weak<T>
// You can also create a weak reference to the value within an Rc<T> instance by calling
// Rc::downgrade() and passing a reference to the Rc<T>, yielding a weak pointer of type
// Weak<T>. To do anything with the value that a Weak<T> is pointing to, you must make
// sure the value still exists by calling the upgrade() method on a Weak<T> instance.
// ------------------------------------------------------------------------------------------------
// As an example, rather than using a list whose items know only about the next item, we’ll
// create a tree whose items know about their children items and their parent items.

#[derive(Debug)]
struct Node {
  // We want a Node to own its children, and we want to share that ownership with variables
  // so we can access each Node in the tree directly. To do this, we define the Vec<T> items
  // to be values of type Rc<Node>. We also want to modify which nodes are children of another
  // node, so we have a RefCell<T> in children around the Vec<Rc<Node>>.
  // ----
  // We also want leaf to know that branch is its parent - if done using stong reference, this
  // yields a circular reference, thus we'll need to use weak reference here
  value: i32,
  parent: RefCell<Weak<Node>>, // a child should not own its parent, thus a weak reference
  children: RefCell<Vec<Rc<Node>>>, // a parent should own its children, thus a strong reference
                               // (if we drop a child node, the parent should still exist, while
                               // if we drop a parent node, children should cease to exist as well)
}

impl Node {
  fn display_ref_count(node: &Rc<Node>, name: &str) {
    println!(
      "{name} strong = {} weak = {}",
      Rc::strong_count(node),
      Rc::weak_count(node)
    );
  }
}

fn prevent_ref_cycle() {
  let leaf = Rc::new(Node {
    value: 3,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![]),
  });

  println!("Leaf is {:?}", &leaf);
  Node::display_ref_count(&leaf, "Leaf");
  println!("------");

  {
    let branch = Rc::new(Node {
      value: 5,
      parent: RefCell::new(Weak::new()),
      children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    println!("Branch is {:?}", &branch);
    Node::display_ref_count(&branch, "Branch");
    Node::display_ref_count(&leaf, "Leaf");
    println!("------");

    println!();
    println!("------");
    println!("Leaf parent is {:?}", leaf.parent.borrow().upgrade());
    println!("Branch parent is {:?}", branch.parent.borrow().upgrade());
    println!("------");

    println!("\n----- Assigning a parent to the leaf -----");
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("Leaf parent is {:?}", leaf.parent.borrow().upgrade());

    println!("------");
    Node::display_ref_count(&branch, "Branch");
    Node::display_ref_count(&leaf, "Leaf");
    println!("------");
  }
  Node::display_ref_count(&leaf, "Leaf");
}

pub fn run() {
  ref_cycle_memleak();
  prevent_ref_cycle();
}
