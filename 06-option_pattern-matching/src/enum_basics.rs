pub fn option_enum_basics() {
  let some_number: Option<i32> = Some(5);
  let some_char: Option<char> = Some('e');
  let absent_number: Option<i32> = None; // for 'None' you must specify Option<TYPE>
  println!("some_number: {:?}", some_number);
  println!("some_char: {:?}", some_char);
  println!("absent_number: {:?}", absent_number);

  // the following are the full versions with Option:: prefix
  // note: Option, Some, and None are added to the Prelude so they don't must be imported

  let some_number = Option::Some(5); // Type can be inferred
  let some_char = Option::Some('e'); // Type can be inferred
  let absent_number: Option<i32> = Option::None;
  println!("some_number: {:?}", some_number);
  println!("some_char: {:?}", some_char);
  println!("absent_number: {:?}", absent_number);

  // Converting Option<T> to T:
  // Docs: https://doc.rust-lang.org/std/option/enum.Option.html
  // Becoming familiar with the methods on Option<T> will be extremely useful in your journey with Rust.
  let some_number_val = match some_number {
    Some(num) => num,
    None => 0,
  };

  let some_char_val = match some_char {
    Some(char) => char,
    None => ' ',
  };

  let absent_number_val = match absent_number {
    Some(num) => num,
    None => -1,
  };

  println!("Value of some_number is {}", some_number_val);
  println!("Value of some_char is {}", some_char_val);
  println!("Value of absent_number is {}", absent_number_val);

  // GENERICS APPROACH to unwrapping the option value:
  // Converting Option<T> to T or a default value by using an unwrap function
  // NOTE: This is a real function in the standard library!
  fn unwrap_or<T>(my_option: Option<T>, default_value: T) -> T {
    match my_option {
      Some(val) => val,
      None => default_value,
    }
  }

  println!("Value of some_number is {}", unwrap_or(some_number, -1));
  println!("Value of some_char is {}", unwrap_or(some_char, ' '));
  println!("Value of absent_number is {}", unwrap_or(absent_number, -1));

  // use a real function in the standard library!
  // https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or
  println!("Value of some_number is {}", some_number.unwrap_or(-1));
  println!("Value of some_char is {}", some_char.unwrap_or(' '));
  println!("Value of absent_number is {}", absent_number.unwrap_or(-1));
}
