// Drop trait lets you customize what happens when a value is about to go out of scope.
// You can provide an implementation for the Drop trait on any type, and that code can
// be used to release resources like files or network connections.
// --------
// We’re introducing Drop in the context of smart pointers because the functionality of
// the Drop trait is almost always used when implementing a smart pointer. For example,
// when a Box<T> is dropped it will deallocate the space on the heap that the box points to.

struct CustomSmartPointer {
  data: String,
}

// std::ops::Drop, but is included in the prelude
impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    println!("> Dropping CustomSmartPointer `{}`", self.data);
  }
}

fn auto_drop() {
  let x = CustomSmartPointer {
    data: "Hello World".to_string(),
  };
  let y = CustomSmartPointer {
    data: "Hello Universe".to_string(),
  };
  println!("{}", x.data);
  println!("{}", y.data);
  println!("CustomSmartPointers created.");
}

fn manual_drop() {
  let x = CustomSmartPointer {
    data: "Hello World".to_string(),
  };
  let y = CustomSmartPointer {
    data: "Hello Universe".to_string(),
  };
  println!("{}", x.data);
  println!("{}", y.data);
  println!("CustomSmartPointers created.");
  println!("Dropping one CustomSmartPointer before the end of main...");
  // c.drop(); // we’re not allowed to explicitly call drop, compiler error
  drop(x); // [std::mem::drop()] available for manual early clean-up
  println!("Dropped."); // x is not available any more, it has MOVED with the 'drop()', consumed, and destroyed
}

pub fn run() {
  println!("----- AUTOMATIC DROP -----");
  auto_drop();
  println!("----- MANUAL DROP -----");
  manual_drop();
}
