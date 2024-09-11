use crate::console;

// Vectors allow you to store more than one value in a single
// data structure that puts all the values next to each other
// in memory. Vectors can only store values of the SAME TYPE.

// VECTOR API: https://doc.rust-lang.org/std/vec/struct.Vec.html

pub fn vectors() {
  console::section("VECTORS:");
  creating_vectors();
  updating_vectors();
  reading_vectors();
  referencing_vectors();
  iterating_over_vectors();
  safe_iterations();
}

fn creating_vectors() {
  // Because we aren’t inserting any values into this vector,
  // we added a type annotation here, as Rust doesn’t know
  // what kind of elements we intend to store...
  let v: Vec<i32> = Vec::new();
  println!("annotated v: {:?}", v);

  // More often, you’ll create a Vec<T> with initial values and
  // Rust will infer the type of value you want to store, so you
  // rarely need to do this type annotation.

  // Rust conveniently provides the vec! macro, which will create
  // a new vector that holds the values you give it.
  let v = vec![1, 2, 3];
  println!("inferred v: {:?}", v);
}

fn updating_vectors() {
  // ADDING values to a vector:
  // let mut v: Vec<i32> = Vec::new();
  let mut v = Vec::new(); // Rust can infer type on the first 'push'
  v.push(3);
  v.push(2);
  v.push(1);
  v.push(0);
  println!("v: {:?}", v);
}

fn reading_vectors() {
  let v = vec![1, 2, 3, 4, 5];
  println!("The whole vector is {:?}", v);

  // via indexing:
  let third: &i32 = &v[2]; // we'll use a pointer rather than copying the value
  println!("The third element is {}", third);

  let third: Option<&i32> = v.get(2); // get gives a pointer rather than copying the value
  match third {
    Some(n) => println!("The third element via get() is {}", n),
    None => println!("The third element does not exist"),
  }

  // ## indexing NOT thread-safe, can ask for an out-of-bounds element
  // let sixth = &v[5];
  // println!("The sixth element is {}", sixth);

  // via get() method:
  println!("The whole vector after reading is {:?}", v);
}

fn referencing_vectors() {
  // Because vectors put the values next to each other in memory, adding
  // a new element onto the end of the vector might require allocating
  // new memory and copying the old elements to the new space, if there
  // isn’t enough room to put all the elements next to each other where
  // the vector is currently stored. In that case, the reference to the
  // first element would be pointing to deallocated memory. The borrowing
  // rules prevent programs from ending up in that situation:

  let mut v = vec![1, 2, 3, 4, 5];
  let _first = &v[0]; // why should a reference to the first element care
  v.push(6); //          about changes at the end of the vector?
  let _first = &v[0]; // without this, the program panics on "immutable borrow"
  println!("The first element is: {_first}");
}

fn iterating_over_vectors() {
  console::spacer();

  // iterate over immutable references
  let v = vec![100, 32, 57];
  println!("The whole vector before iterating is {:?}", v);
  for n_ref in &v {
    let n_plus_one = *n_ref + 1; // n_ref has type &i32 (from &v?)
    print!("{}", n_plus_one);
    let n_plus_one = n_ref + 1; // n_ref has type &i32, but this works too?!
    println!(" -- {}", n_plus_one);
  }
  println!("The whole vector after iterating is {:?}", v);

  // iterate over mutable references
  println!("The whole vector before iterating is {:?}", v);
  let mut v = vec![100, 32, 57];
  for n_ref in &mut v {
    // n_ref has type &mut i32
    *n_ref += 20; // here, a pointer must be used, unlike above
  }
  println!("The whole vector after iterating is {:?}", v);
}

fn safe_iterations() {
  // using Iterator to iterate safely
  use std::slice::Iter;
  let v: Vec<i32> = vec![1, 2];
  let mut iter: Iter<'_, i32> = v.iter();
  let n1: &i32 = iter.next().unwrap();
  let n2: &i32 = iter.next().unwrap();
  let end: Option<&i32> = iter.next();
  println!("{} {} {:?}", *n1, *n2, end);

  // using Range to iterate safely
  use std::ops::Range;
  let v: Vec<i32> = vec![1, 2];
  let mut iter: Range<usize> = 0..v.len();
  let i1: usize = iter.next().unwrap(); // iterates over a lenght range, not the Vector directly
  let n1: &i32 = &v[i1]; //                accesses using index; safety comes from the range
  println!("{} {:?}", *n1, i1);
}
