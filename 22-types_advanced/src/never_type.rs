// Rust has a special type named ! that’s known in type theory lingo as the empty type because
// it has no values. We prefer to call it the never type because it stands in the place of the
// return type when a function will never return. Here is an example:

// This code is read as “the function bar returns never.”
//   fn bar() -> ! {
//       // --snip--
//   }

// Functions that return never are called diverging functions. We can’t create values of the
// type ! so bar can never possibly return. The common use is in the `contiue` expression
// which returns `!`, and Rust can decide to treat it as the whichever type it needs
// (think match arms where continue can coerce ! into the type of the arms that return value)
//
//   let guess: u32 = match guess.trim().parse() {
//       Ok(num) => num,
//       Err(_) => continue,
//   };
//
