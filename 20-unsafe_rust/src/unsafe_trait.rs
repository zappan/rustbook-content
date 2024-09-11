// A trait is unsafe when at least one of its methods has some invariant that
// the compiler can’t verify. We declare that a trait is unsafe by adding the
// unsafe keyword before trait and marking the implementation of the trait as unsafe too

unsafe trait Foo {
  // methods go here
  fn unsafe_add(a: i32, b: i32) -> i32;
}

unsafe impl Foo for i32 {
  // method implementations go here

  // this is not an unsafe implementation, but just to show the syntax
  fn unsafe_add(a: i32, b: i32) -> i32 {
    a + b
  }
}

pub fn run() {}
