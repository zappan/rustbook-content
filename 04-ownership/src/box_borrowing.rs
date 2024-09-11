// A REFERENCE is a kind of pointer. The expression &m1 uses the ampersand
// operator to create a reference to (or "BORROW") m1. The type of the greet
// parameter g1 is changed to &String, meaning "a reference to a String".

// taking in params with ampersands means "borrowing from" the caller
fn greet(g1: &String, g2: &String) {
  // g1 is a reference that points to m1 on the stack, and m1 is a String
  // containing a box that points to "Hello" on the heap.

  println!("{} {}!", g1, g2);

  // While m1 owns the heap data "Hello", g1 does not own either m1 or "Hello".
  // Therefore after greet() ends, no heap data has been deallocated.

  // Only the stackframe for greet disappears. This fact is consistent with our Box Deallocation Principle.
  // REFERENCES are NON-OWNING POINTERS, because they do not own the data they point to.
}

fn strings_borrowing() {
  let m1: String = String::from("Hello");
  let m2: String = String::from("world");
  greet(&m1, &m2); // passing addresses -- references => borrowing them to 'greet()'

  // Because g1 did not own "Hello", Rust did not deallocate "Hello" on behalf of g1.
  let s: String = format!("{} {} from the main", m1, m2);
  println!("{s}");
}

// ----------------------------------------------------------------------------------------
// Pointers can be created either through boxes (pointers owning data on the heap)
// or references (non-owning pointers).
// ----------------------------------------------------------------------------------------
fn ints_borrowing() {
  let mut x: Box<i32> = Box::new(1);
  let a: i32 = *x; // dereferencing a pointer (access es the data)

  // x += 1; // cannot update the 'Box'
  *x += 1; // must update the value behind the 'Box'

  let ref1: &Box<i32> = &x; // a reference to the Box
  let b: i32 = **ref1; // a reference to the reference (the Box) of the value x

  let ref2: &i32 = &*x; // a(nother) reference to the value of x
  let c: i32 = *ref2; // assign the value of x to c

  // *x += 1; // "cannot do another assign to *x because it was already borrowed" (with ... = &x)

  println!("{x}"); // boxed
  println!("{a}"); // unboxed
  println!("{ref1}"); // double-boxed => a reference to a box
  println!("{b}"); // unboxed
  println!("{c}"); // unboxed
}

// You probably won't see the dereference operator very often when you read Rust code.
// Rust implicitly inserts dereferences and references in certain cases, such as calling
// a method with the dot operator. For example, this program shows two equivalent ways of
// calling the i32::abs (absolute value) and str::len (string length) functions:
fn dereferencing() {
  let x: Box<i32> = Box::new(-1);

  let x_abs1: i32 = i32::abs(*x); // explicit dereference
  let x_abs2: i32 = x.abs(); // implicit dereference
                             //  The dot syntax is syntactic sugar for the function-call syntax.

  assert_eq!(x_abs1, x_abs2);
  println!("{x}, {x_abs1}, {x_abs2}");

  let ref_x: &Box<i32> = &x;
  let ref_abs1: i32 = i32::abs(**ref_x); // explicit double dereference
  let ref_abs2: i32 = ref_x.abs(); // implicit double dereference

  assert_eq!(ref_abs1, ref_abs2);
  println!(
    "Absolute values of references to x are {} and {}",
    ref_abs1, ref_abs2
  );

  let s: String = String::from("Hello");
  let s_len1: usize = str::len(&s); // explicit reference
  let s_len2: usize = s.len(); // implicit reference

  assert_eq!(s_len1, s_len2);
  println!(
    "Strings lengths l_len1 and s_len2 are {} and {}, respectively",
    s_len1, s_len2
  );
}

pub fn ownership_borrowing() {
  strings_borrowing();
  ints_borrowing();
  dereferencing();
}
