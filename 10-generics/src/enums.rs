// my implementation of Option<T> for demonstration of enum-generics
#[derive(Debug)]
enum MyOption<T> {
  SomeValue(T),
  NoValue,
}

// my implementation of Result<T, E> for demonstration of enum-generics
#[derive(Debug)]
enum ResultWrapper<T, E> {
  Success(T),
  Fail(E),
}

impl<T: std::fmt::Debug> MyOption<T> {
  fn print(&self) {
    match self {
      MyOption::SomeValue(val) => println!("Wrapped option value: {:?}", val),
      MyOption::NoValue => println!("Wrapped optin novalue: {:?}", self),
    }
  }
}

impl<T: std::fmt::Debug, E: std::fmt::Debug> ResultWrapper<T, E> {
  fn print(&self) {
    match self {
      ResultWrapper::Success(val) => println!("Wrapped result: {:?}", val),
      ResultWrapper::Fail(err) => println!("Wrapped error: {:?}", err),
    }
  }
}

pub fn main() {
  let optional_value: MyOption<i32> = MyOption::SomeValue(12);
  let optional_novalue: MyOption<i32> = MyOption::NoValue;
  optional_value.print();
  optional_novalue.print();
  println!(
    "Optional value & non-value: {:?}, {:?}",
    optional_value, optional_novalue
  );

  println!("------------------------");

  let wrapped_result: ResultWrapper<char, std::fmt::Error> = ResultWrapper::Success('c');
  let wrapped_error: ResultWrapper<char, std::fmt::Error> =
    ResultWrapper::Fail(std::fmt::Error::default());

  wrapped_result.print();
  wrapped_error.print();
  println!(
    "Wrapped result & error: {:?}, {:?}",
    wrapped_result, wrapped_error
  );
}
