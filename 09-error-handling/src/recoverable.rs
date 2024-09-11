use std::error::Error as StdError;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::net::IpAddr;

// Note that, like the Option enum, the Result enum and its variants
// have been brought into scope by the prelude, so we don’t need to
// specify Result:: before the Ok and Err variants in the match arms.

pub fn main() -> Result<(), Box<dyn StdError>> {
  matching_on_errors();
  panic_on_error_shortcuts();
  error_propagation();
  shorthand_operator_on_option_t()?;

  // When a main function returns a Result<(), E>, the executable will exit with
  // a value of 0 if main returns Ok(()) and will exit with a nonzero value if main
  // returns an Err value. Executables written in C return integers when they exit:
  // programs that exit successfully return the integer 0, and programs that error
  // return some integer other than 0. Rust also returns integers from executables
  // to be compatible with this convention.
  Ok(())
}

fn matching_on_errors() {
  // opening a file may result in an error:
  let filename = "hello.txt";
  let fileopen_result: Result<File, Error> = File::open(filename);

  // println!("{:?}", fileopen_result);
  // println!("We'll try to create a file if it doesn't exist...");

  let _file: File = match fileopen_result {
    Ok(file) => file,
    Err(err) => match err.kind() {
      ErrorKind::NotFound => match File::create(filename) {
        Ok(file) => file,
        Err(err) => panic!("Can't create the file: {:?}\n", err),
      },
      other_error => panic!("Can't open the file: {:?}\n", other_error),
    },
  };

  // the above are the primitives; we can use closures and unwrap to do the same:
  let filename = "hello-again.txt";
  let _file = File::open(filename).unwrap_or_else(|err| {
    if err.kind() == ErrorKind::NotFound {
      File::create(filename).unwrap_or_else(|err| {
        panic!("Can't create the file: {:?}\n", err);
      })
    } else {
      panic!("Can't open the file: {:?}\n", err);
    }
  });

  // println!("File available")
  // we can now use file for reading and writing
}

fn panic_on_error_shortcuts() {
  // let home_ip: IpAddr = "1271.0.0.1"
  let home_ip: IpAddr = "127.0.0.1"
    .parse()
    .expect("Hardcoded IP address should be valid");
  println!("Home IP address is: {}\n", home_ip);

  // ### Shortcuts for Panic on Error: unwrap and expect
  // ---------------------------------------------------
  // If the Result value is the Ok variant, unwrap() will return the value inside the Ok.
  // If the Result is the Err variant, unwrap() will call the panic! macro for us.
  // Similarly, the expect() method lets us also choose the panic! error message.
  // ---------------------------------------------------
  // Here is an example of unwrap() and expect() in action:
  fn unwrap(filename: &str, file_status_msg: &str) {
    println!("unwrap() shortcut when file {}:", file_status_msg);
    let file = File::open(filename).unwrap();
    println!("{:?}\n", file);
  }

  fn expect(filename: &str, file_status_msg: &str) {
    println!("expect() shortcut when file {}:", file_status_msg);
    let err_msg = format!("{} should be included in this project\n", filename);
    let file = File::open(filename).expect(&err_msg);
    println!("{:?}\n", file);
  }

  let filename = "hello.txt";
  unwrap(&filename, "exists");
  expect(&filename, "exists");

  // let filename = "hello-world.txt";
  // unwrap(filename, "does not exist");
  // expect(filename, "does not exist");

  // In production-quality code, most Rustaceans choose expect rather than unwrap
  // and give more context about why the operation is expected to always succeed.
  // That way, if your assumptions are ever proven wrong, you have more information
  // to use in debugging.
}

fn error_propagation() {
  println!("Reading content from file and returning that to the caller");

  let filename = "username-not-found.txt";
  match match_error_propagation(&filename) {
    Ok(username) => println!("Username is: {}", username),
    Err(err) => println!("Error: {:?}", err),
  };

  match shorthand_error_propagation(&filename) {
    Ok(username) => println!("Username is {}", username),
    Err(err) => println!("Error: {:?}", err),
  }

  let filename = "username.txt";
  match match_error_propagation(&filename) {
    Ok(username) => println!("Username is: {}", username),
    Err(err) => println!("Error: {:?}", err),
  };

  match shorthand_error_propagation(&filename) {
    Ok(username) => println!("Username is: {}", username),
    Err(err) => println!("Error: {:?}", err),
  }

  match shorthand_error_propagation_shorter(&filename) {
    Ok(username) => println!("Username is: {}", username),
    Err(err) => println!("Error: {:?}", err),
  }

  match shorthand_error_propagation_shortest(&filename) {
    Ok(username) => println!("Username is: {}", username.trim()),
    Err(err) => println!("Error: {:?}", err),
  }
}

// The pattern of propagating errors is so common in Rust that Rust
// provides the question mark operator ? to make this easier.
fn match_error_propagation(filename: &str) -> Result<String, io::Error> {
  // ## manual error propagation
  let username_file_result = File::open(filename);
  let mut username_file = match username_file_result {
    Ok(file) => file,
    Err(err) => return Err(err), // returns aka propagates an error to the caller
  };

  // ## manual error propagation
  let mut username: String = String::new();
  match username_file.read_to_string(&mut username) {
    Ok(_) => Ok(username.trim().to_string()),
    Err(err) => return Err(err), // returns aka propagates an error to the caller
  }
}

// ## shorthand error propagation using ?
fn shorthand_error_propagation(filename: &str) -> Result<String, Error> {
  // File::open()? if Ok() gives username_file
  // If the value is an Err, the Err will be returned from the whole function as if we
  // had used the return keyword so the error value gets propagated to the calling code.
  let mut username_file: File = File::open(filename)?;

  // which can then try the read_to_string()? that gives username if Ok
  // and, again, if Err, it returns/propagates it to the calling code
  let mut username: String = String::new();
  username_file.read_to_string(&mut username)?;
  Ok(username.trim().to_string())
}

// ## shorthand error propagation using combined ?s
fn shorthand_error_propagation_shorter(filename: &str) -> Result<String, Error> {
  // all the above can be further shortened to:
  let mut username: String = String::new();
  File::open(filename)?.read_to_string(&mut username)?;
  Ok(username.trim().to_string())

  // There is a difference between what the match expression from match_error_propagation()
  // does and what the ? operator does: error values that have the ? operator called
  // on them go through the from function, defined in the From trait in the standard
  // library, which is used to convert values from one type into another. When the ?
  // operator calls the from function, the error type received is converted into the
  // error type defined in the return type of the current function. This is useful
  // when a function returns one error type to represent all the ways a function
  // might fail, even if parts might fail for many different reasons.
  // -----------
  // For example, we could change the read_username_from_file function to return
  // a custom error type named OurError that we define. If we also define impl
  // From<io::Error> for OurError to construct an instance of OurError from an
  // io::Error, then the ? operator calls in the body of read_username_from_file
  // will call from and convert the error types without needing to add any more
  // code to the function.
}

// ## The shorthest error propagation using fs::read_to_string()
// ## Of course, using fs::read_to_string doesn’t give us the opportunity
// ## to explain all the error handling, so we did it the longer ways first.
fn shorthand_error_propagation_shortest(filename: &str) -> Result<String, Error> {
  fs::read_to_string(filename)
}

// The behavior of the ? operator when called on an Option<T> is similar to its
// behavior when called on a Result<T, E>: if the value is None, the None will be
// returned early from the function at that point. If the value is Some, the value
// inside the Some is the resulting value of the expression and the function continues.
fn last_char_of_first_line(text: &str) -> Option<char> {
  // you can only use ? on Option in a function that returns an Option -- LIKE HERE
  text.lines().next()?.chars().last()
}

fn get_text(filename: &str) -> Result<String, Error> {
  shorthand_error_propagation_shortest(&filename)
}

fn shorthand_operator_on_option_t() -> Result<(), Error> {
  println!("----------------------------------------------------");

  let text = get_text("usernames.txt")?;
  match last_char_of_first_line(&text) {
    Some(last_char) => println!("Last char of the file is: {}", last_char),
    None => println!("We didn't find the file?"),
  }

  let text = get_text("usernames-file-empty-first-line.txt")?;
  match last_char_of_first_line(&text) {
    Some(last_char) => println!("Last char of the file is: {}", last_char),
    None => println!("The first line of the file is empty..."),
  };

  let text = get_text("usernames-file-empty.txt")?;
  match last_char_of_first_line(&text) {
    Some(last_char) => println!("Last char of the file is: {}", last_char),
    None => println!("The first line of the file is empty..."),
  };

  let text = get_text("usernames-file-missing.txt")?;
  match last_char_of_first_line(&text) {
    Some(last_char) => println!("Last char of the file is: {}", last_char),
    None => println!("We didn't find the file?"),
  }

  // let text = get_text("usernames-file-missing.txt")?;
  // match shorthand_error_propagation_shortest(&filename) {
  //   Ok(text) => match last_char_of_first_line(&text) {
  //     Some(last_char) => println!("Last char of the file is: {}", last_char),
  //     None => println!("We didn't find the file?"),
  //   },
  //   Err(err) => println!("Error reading file: {:?}", err),
  // }

  Ok(())
}
