// https://rust-book.cs.brown.edu/ch19-04-advanced-types.html
// --------------------------------------------------------------------------------------------------------
// By using this method, we don’t get the type checking benefits that we get from the newtype pattern.
// In other words, if we mix up Kilometers and i32 values somewhere, the compiler will not give us an error.
// The main use case for type synonyms is to reduce repetition. For example, we might have a lengthy type like
//
//   Box<dyn Fn() + Send + 'static>
//
// that can be shortened in the use of that type in function signatures and type annotations (thunk is a word
// for code to be evaluated at a later time, so it’s an appropriate name for a closure that gets stored):
//
//   type Thunk = Box<dyn Fn() + Send + 'static>;
//
// The type alias helps in two ways: it makes code easier to write and it gives us a consistent interface
// across all the code where it's used, like in the following example
//
//   type Result<T> = std::result::Result<T, std::io::Error>;
//   <...>
//     fn write(&mut self, buf: &[u8]) -> Result<usize>;
//     fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
//
// Because it’s an alias, it’s just another Result<T, E>, which means we can use any methods that work on
// Result<T, E> with it, as well as special syntax like the ? operator.

// --------

// Rust provides the ability to declare a type alias to give an existing type another name.
// For this we use the type keyword. Now, the alias Kilometers is a synonym for i32. Values
// that have the type Kilometers will be treated the same as values of type i32:
type Kilometers = i32;

pub fn run() {
  let x: i32 = 5;
  let kms: Kilometers = 21;

  println!("x + y = {}", x + kms);
}
