use crate::util::console;

// A TUPLE is a general way of GROUPING TOGETHER A NUMBER OF VALUES WITH A VARIETY OF TYPES
// into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.
// ----------------------------------------------------------------------------------------------------------
// The tuple without any values has a special name, unit. This value and its corresponding type
// are both written () and represent an empty value or an empty return type.
// Expressions implicitly return the unit value if they don’t return any other value.
fn tuples() {
  console::section("Tuples:");

  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let implicit_tup = (200, 3.1, 2);

  let (a, b, c) = tup; // use pattern matching to destructure a tuple value
  println!("The a, b, and c values of a tuple tup are {a}, {b}, and {c}, respectively");

  let (a, b, c) = implicit_tup;
  println!("The a, b, and c values of a tuple implicit_tup are {a}, {b}, and {c}, respectively");

  // We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access.
  println!("First value of a tuple is: {:?}", tup.0);
  println!("Second value of a tuple is: {:?}", tup.1);
  println!("Third value of a tuple is: {:?}", tup.2);

  // Additionally, we can modify individual elements of a mutable tuple.
  let mut tup: (i32, i32) = (1, 2);
  println!(
    "Mutable tuple initial values are ({:?}, {:?})",
    tup.0, tup.1
  );

  tup.0 = -5;
  tup.1 -= 21;
  println!(
    "Mutable tuple values after mutation are ({:?}, {:?})",
    tup.0, tup.1
  );
}

// Unlike arrays in some other languages, arrays in Rust have a fixed length.
// Unlike a tuple, every element of an array must have the same type.
// ---------------------------------------------------------------------------------
// Arrays are useful when you want your data allocated on the stack rather than the heap
// or when you want to ensure you always have a fixed number of elements.
// ---------------------------------------------------------------------------------
// An array isn’t as flexible as the vector type, though. A vector is a similar collection
// type provided by the standard library that is allowed to grow or shrink in size.
// If you’re unsure whether to use an array or a vector, chances are you should use a vector.
fn arrays() {
  console::section("Arrays:");

  // arrays are more useful when you know the number of elements will not need to change
  let months = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
  ];
  println!("Names of months are: {:?}", months);
  println!("Name of the seventh month is: {:?}", months[6]);

  // what happens if you try to access an element of an array that is past the end of the array?
  // println!("Name of the thirteenth month is: {:?}", months[12]); // will throw a compiler error
  // if index is dynamic, the program will panic with out-of-bounds runtime error
  // --------------------------------------------------------------------------------------------------
  // THIS IS AN EXAMPLE OF RUST’S MEMORY SAFETY PRINCIPLES IN ACTION
  // Rust protects you against invalid memory access by immediately exiting instead of allowing the
  // memory access and continuing, like many other low-level languages do without such a check in place.

  // explicitly indicating array type:
  let a: [i32; 5] = [1, 2, 3, 4, 5];
  println!("Explicitly indicated array's values are: {:?}", a);

  // intialize an array to contain the same element
  let a = [3; 7];
  println!("Array initialized with the same value: {:?}", a);
}

fn array_tuple_access() {
  console::section("Array & Tuple access:");

  let t = ([1; 2], [3; 4]);
  let (a, _b) = t;
  println!("sum a[0] + t.1[0] is {}", a[0] + t.1[0]);
}

pub fn compounds() {
  tuples();
  arrays();
  array_tuple_access();
}
