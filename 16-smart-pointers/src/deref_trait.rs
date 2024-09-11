fn dereference_box() {
  let x = 5;
  let y = Box::new(5);
  assert_eq!(x, 5);
  assert_eq!(*y, 5);
  assert_eq!(x, *y);
  println!("x={}, *y={}", x, *y);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}

impl<T> std::ops::Deref for MyBox<T> {
  type Target = T;

  // The reason the deref method returns a reference to a value is to do with the ownership system.
  // If the deref method returned the value directly instead of a reference to the value, the value
  // would be moved out of self. We don’t want to take ownership of the inner value inside MyBox<T>.
  fn deref(&self) -> &Self::Target {
    &self.0 // MyBox<T> is a struct of a single tuple, accessed here with .0
  }
}

fn dereference_mybox() {
  let x = 5;
  let y = MyBox::new(5);
  assert_eq!(x, 5);
  assert_eq!(*y, 5);
  assert_eq!(x, *y);
  println!("x={}, *y={}", x, *y);
}

// Deref coercion converts a reference to a type into a reference to another type.
// Deref coercion is a convenience Rust performs on arguments to functions and methods,
// and works only on types that implement the Deref trait.
// For example, deref coercion can convert &String to &str because String
// implements the Deref trait such that it returns &str.
fn dereference_coercion() {
  // # expects a string slice!!!
  fn hello(name: &str) {
    println!("Hello, {}", name);
  }

  let m = MyBox::new(String::from("Rust")); // a String

  // Rust can turn &MyBox<String> into &String by calling deref. The standard library
  // provides an implementation of Deref on String that returns a string slice
  hello(&m); // deref-coerced into string slice, then borrowed to hello()

  // # If Rust didn’t implement deref coercion,  to call hello(), we would have to write the following:
  hello(&(*m)[..]); // *m derefernce into a String; [..] taking it as a slice; &-borrow to match hello()
}

pub fn run() {
  dereference_box();
  dereference_mybox();
  dereference_coercion();
  // Similar to how you use the Deref trait to override the * operator on immutable references,
  // you can use the DerefMut trait to override the * operator on mutable references.
}
