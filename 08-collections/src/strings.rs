use crate::console;

// Strings are implemented as a collection of bytes, plus some methods to
// provide useful functionality when those bytes are interpreted as text.

// Rust has only one string type in the core language, which is the
// STRING SLICE str that is usually seen in its borrowed form &str.
// String slices are references to some UTF-8 encoded string data stored
// elsewhere. String literals, for example, are stored in the program’s
// binary and are therefore string slices.

// The String type, which is provided by Rust’s standard library rather than coded
// into the core language, is a growable, mutable, owned, UTF-8 encoded string type.

// ------------------------------------------------------------------------------
// When Rustaceans refer to “strings” in Rust, they might be referring to either
// the String or the string slice &str types, not just one of those types.
// Although this section is largely about String, both types are used heavily in
// Rust’s standard library, and both String and string slices are UTF-8 encoded.
// ------------------------------------------------------------------------------

// String is actually implemented as a wrapper around a vector of bytes
// with some extra guarantees, restrictions, and capabilities, thus many of
// the same operations available with Vec<T> are available with String as well

pub fn strings() {
  console::section("STRINGS:");
  string_creation();
  string_updates();
  string_concatenation();
  indexing_into_strings();
  slicing_strings();
  iterating_over_strings();
}

fn string_creation() {
  let mut s = String::new();
  println!("Empty String: {:?}", s);

  // Often, we’ll have some initial data that we want to start the string with. For that,
  // we use the to_string method, available on any type that implements the Display trait
  let data = "Initial contents";
  s = data.to_string();
  println!("Non-empty, preinitialized string: {:?}", s);

  // the method also works on a literal directly:
  s = "initial contents".to_string();
  println!("Non-empty, preinitialized string: {:?}", s);

  // We can also use the function String::from to create a String
  // from a string literal; equivalend to the two above ways
  s = String::from("initial contents");
  println!("Preinitialized using String::from(): {:?}", s);
}

fn string_updates() {
  // grow a String by using the push_str method to append a string slice
  let mut s = String::from("foo");
  s.push_str("bar");
  println!("push_str-ed string: {:?}", s);

  // The push_str method takes a string slice because we don’t necessarily
  // want to take ownership of the parameter
  // => we want to be able to use s2 after appending its contents to s1
  let mut s1 = String::from("foo");
  let s2 = "bar";
  s1.push_str(s2);
  println!("s2 is {s2}",);

  // The push method takes a single character as a param and adds it to the String
  let mut s = String::from("lo");
  s.push('l');
  println!("s is {s}",);

  // Often to combine two existing strings, a way to do so is to use the + operator
  let s1 = String::from("Hello, ");
  let s2 = String::from("world!");
  let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
                     // while s2 is required to be passed as borrowed

  // The + operator uses the add method, whose signature looks something like this:
  // fn add(self, s: &str) -> String {...}

  // println!("s1 is {s1}",);  // can't get to it any more, the above operator took it
  println!("s2 is {s2}",);
  println!("s3 is {s3}",);
}

fn string_concatenation() {
  // If we need to concatenate multiple strings, the behavior of the + operator gets unwieldy:
  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");

  let s = s1 + "-" + &s2 + "-" + &s3;
  println!("{s}",);

  // For more complicated string combining, we can instead use the format! macro:
  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");

  let s = format!("{s1}-{s2}-{s3}");
  println!("{s}",);

  // Also, the code generated by the format! macro uses references so that
  // this call doesn’t take ownership of any of its parameters.
  println!("{s1}, {s2}, {s3}",);
}

fn indexing_into_strings() {
  let _s1 = String::from("hello");
  // let h = s1[0] // compiler error, Rust doesn't allow this

  // REASON: String is a wrapper over a Vec<u8>; while latin characters
  // take 1-byte in UTF-8, cyrillic, arabic, and others take 2 bytes per char
  // making it impossible to access an exact character via an index, as chars
  // may have different length, affecting the index.
  // Therefore, an index into the string’s bytes will not always correlate
  // to a valid Unicode scalar value.

  let _hello = String::from("Hola"); // 4 latin chars => 4-bytes long
  let _hello = String::from("Здравствуйте"); // 12 cyrillic chars => 24 bytes long
                                             // (Hindi takes 3 bytes to encode a character in UTF-8)

  // With UTF-8, there are actually three relevant ways to look at strings
  // from Rust’s perspective: as bytes, scalar values, and grapheme clusters
  // (the closest thing to what we would call letters).
  // -------------------------------------------------------------------------------
  // A final reason Rust doesn’t allow us to index into a String to get a character
  // is that indexing operations are expected to always take constant time (O(1)).
  // But it isn’t possible to guarantee that performance with a String, because
  // Rust would have to walk through the contents from the beginning to the index
  // to determine how many valid characters there were.
}

fn slicing_strings() {
  // Rather than indexing using [] with a single number, you can use [] with
  // a range to create a string slice containing particular bytes:
  let hello = "Здравствуйте";
  let s = &hello[2..4]; // gets bytes 2 & 3, equivalent to the letter 'д'
  println!("Sliced string is: {:?}", s);
  let s = &hello[0..4]; // gets bytes 0, 1 2 & 3, equivalent to the letters 'Зд'
  println!("Sliced string is: {:?}", s);

  // ## If we were to try to slice only part of a character’s bytes with something
  // ## like &hello[0..1], Rust would panic at runtime in the same way as if an
  // ## invalid index were accessed in a vector:
  // let s = &hello[0..1]; // gets byte 0, which isn't a char boundary => PANIC
  // println!("Sliced string is: {:?}", s);

  // Use ranges to create string slices with caution, because doing so can crash your program.
}

// The best way to operate on pieces of strings is to be explicit about whether you want
// characters or bytes. For individual Unicode scalar values, use the chars method.
fn iterating_over_strings() {
  let hello = "Здравствуйте";

  // Calling chars on “Зд” separates out and returns two values of type char,
  // and you can iterate over the result to access each element:
  for c in hello.chars() {
    print!("{c}");
  }
  println!();

  // Alternatively, the bytes method returns each raw byte,
  // which might be appropriate for your domain:
  for b in hello.bytes() {
    print!("{b}-");
  }
  println!();

  // Getting grapheme clusters from strings as with the Devanagari script is
  // complex, so this functionality is not provided by the standard library.
  // Crates are available on crates.io if this is the functionality you need.
}
