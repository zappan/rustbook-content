pub fn add(left: u64, right: u64) -> u64 {
  left + right
}

pub fn add_two(a: i32) -> i32 {
  a + 2
}

pub fn greeting(name: &str) -> String {
  format!("Hello, {name}!")
}

fn prints_and_returns_10(a: i32) -> i32 {
  println!("I got the value {}", a);
  10
}

#[derive(Debug)]
struct Rectangle {
  width: i32,
  height: i32,
}

impl Rectangle {
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

pub struct Guess {
  value: i32,
}

impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < 1 {
      panic!("Guess value must be greater than or equal 1, got {value}.");
    } else if value > 100 {
      panic!("Guess value must be less than or equal 100, got {value}.");
    }

    Guess { value }
  }
}

// You’ll put unit tests in the src directory in each file with the code that they’re
// testing. The convention is to create a module named tests in each file to contain
// the test functions and to annotate the module with cfg(test).
#[cfg(test)]
mod tests {
  use super::*;

  // #[test]
  // fn it_works_not() {
  //   panic!("Make this test fail!");
  // }

  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }

  // Writing tests so they return a Result<T, E> enables you to use the question
  // mark operator in the body of tests, which can be a convenient way to write
  // tests that should fail if any operation within them returns an Err variant.
  // ----------------------------------------------------------------------------
  // You can’t use the #[should_panic] annotation on tests that use Result<T, E>.
  // ----------------------------------------------------------------------------
  // To assert that an operation returns an Err variant, don’t use the question
  // mark operator on the Result<T, E> value. Instead, use assert!(value.is_err()).
  #[test]
  fn it_works_via_result() -> Result<(), String> {
    if 2 + 2 == 4 {
      Ok(())
    } else {
      Err(String::from("two plus two does not equal four"))
    }
  }

  #[test] // annotation: attribute indicating a test
  fn it_adds() {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }

  #[test]
  fn it_adds_two() {
    assert_eq!(add_two(4), 6);
  }

  #[test]
  fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
      result.contains("Carol"),
      "Greeting did not contain name, value was '{}'",
      result
    );
  }

  #[test]
  fn larger_can_hold_smaller() {
    let larger = Rectangle {
      width: 8,
      height: 7,
    };
    let smaller = Rectangle {
      width: 5,
      height: 1,
    };

    assert!(larger.can_hold(&smaller));
  }

  #[test]
  fn smaller_cannot_hold_larger() {
    let larger = Rectangle {
      width: 8,
      height: 7,
    };
    let smaller = Rectangle {
      width: 5,
      height: 1,
    };
    assert!(!smaller.can_hold(&larger));
  }

  #[test]
  #[should_panic]
  fn greater_than_100() {
    Guess::new(200);
  }

  #[test]
  #[should_panic(expected = "less than or equal 100")] // matches a substring of the panic message
  fn greater_than_100_expected() {
    Guess::new(200);
  }

  // silly fn, but for showing config flag on printing out the output during test runs
  #[test]
  fn this_test_will_pass() {
    let value = prints_and_returns_10(4);
    assert_eq!(10, value);
  }

  #[test]
  #[ignore]
  fn this_test_will_fail() {
    let value = prints_and_returns_10(8);
    assert_eq!(5, value);
  }

  // showcasing ignoring of a test
  #[test]
  #[ignore]
  fn expensive_test() {
    // code that takes an hour to run
  }
}
