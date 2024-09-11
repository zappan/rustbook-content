// Vectors can only store values that are the same type. This can be
// inconvenient; there are definitely use cases for needing to store
// a list of items of different types. Fortunately, the variants of an
// enum are defined under the same enum type, so when we need one type
// to represent elements of different types, we can define and use an enum!

// If you don’t know the exhaustive set of types a program will get at
// runtime to store in a vector, the enum technique won’t work.
// Instead, you can use a trait object (covered later in Chapter 17)

#[derive(Debug)]
enum SpreadsheetCell {
  Int(i32),
  Float(f64),
  Text(String),
}

// hiding different types under a wrapping enum type
pub fn enum_vectors() {
  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(34.21),
  ];
  println!("{:?}", row);
}
