use crate::util::console;

// ----------------------------------------------------------------------------
// Slices let you reference a contiguous sequence of elements in a collection
// rather than the whole collection.
// ----------------------------------------------------------------------------
// A slice is a kind of reference, so it is a non-owning pointer.
// ----------------------------------------------------------------------------
// Slices are special kinds of references because they are "fat" pointers,
// or pointers with metadata. Here, the metadata is the length of the slice.
// ----------------------------------------------------------------------------

// we ask for a "borrowed" string, we don't want ownership
fn first_word_iterative(s: &String) -> usize {
  let bytes = s.as_bytes();

  for (i, &byte_by_byte) in bytes.iter().enumerate() {
    if byte_by_byte == b' ' {
      return i;
    }
  }

  return s.len();
}

// The type that signifies “string slice” is written as &str:
fn first_word_slices(s: &str) -> &str {
  // println!("{}", s.len())
  let bytes: &[u8] = s.as_bytes();

  for (i, &byte_by_byte) in bytes.iter().enumerate() {
    if byte_by_byte == b' ' {
      return &s[..i];
    }
  }

  &s[..]
}

// String Literals Are Slices
fn string_literals() {
  // Recall that we talked about string literals being stored inside the [compiled program] binary.
  // Now that we know about slices, we can properly understand string literals:

  // The type of s here is &str: it’s a slice pointing to that specific point of the binary.
  // This is also why string literals are immutable; &str is an immutable reference.
  let s = "Remember, string literals are slices!";
  println!("{s}");
}

fn int_slices() {
  let a: [i32; 5] = [1, 2, 3, 4, 5];
  let slice = &a[1..3];
  assert_eq!(&[2, 3], slice);
  println!("Slices also support non-string types: {:?}", slice);
}

fn slice_pointer_size() {
  println!(
    "Size of a regular String pointer (&String) is {} bytes, while size of a string slice pointer (&str) is {} bytes",
    std::mem::size_of::<&String>(),
    std::mem::size_of::<&str>(),
  );
}

pub fn slices() {
  // WE ARE ASSUMING ASCII (byte-length) CHARACTERS IN THIS SLICE EXAMPLE
  // UTF-8 CHARACTERS WOULD YIELD A PANIC IN THE CODE BELOW...
  // UTF-8 handling is discussed at a future point.
  //                                      1
  //                            01234567890
  let s: String = String::from("Hello world");
  let size: usize = first_word_iterative(&s);
  println!("{size}");
  console::spacer();

  // Because slices are references, they also change the permissions on referenced data.
  let hello = &s[0..5]; // when hello is created as a slice of s, then s loses write and own permissions:
  let world = &s[6..11];
  println!("{hello} {world}");

  // alternative RANGE syntax where the rirst and the last index may be dropped
  let hello = &s[..5];
  let world = &s[6..];
  println!("{hello} {world}");

  // you may drop both the leading and the trailing numbers => takes the entire string into the range
  let slice: &str = &s[..];
  println!("{slice}");
  console::spacer();

  let first_word: &str = first_word_slices(&s);
  println!("{first_word}");

  let s2: &String = &s;
  let first_word: &str = first_word_slices(s2);
  println!("{first_word}");

  let my_string_literal: &str = "Hello my string literal";
  let first_word: &str = first_word_slices(my_string_literal);
  println!("{first_word}");
  let first_word: &str = first_word_slices(&my_string_literal);
  println!("{first_word}");
  let first_word: &str = first_word_slices(&my_string_literal[6..17]);
  println!("{first_word}");
  console::spacer();

  string_literals();
  int_slices();
  slice_pointer_size();
}
