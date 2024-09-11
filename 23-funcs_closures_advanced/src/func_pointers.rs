// Function pointers implement all three of the closure traits (Fn, FnMut, and FnOnce), meaning
// you can always pass a function pointer as an argument for a function that expects a closure.
// -------
// It’s best to write functions using a generic type and one of the closure traits so your
// functions can accept either functions or closures.

fn add_one(x: i32) -> i32 {
  x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
  f(arg) + f(arg)
}

pub fn func_ptr() {
  let result = do_twice(add_one, 5);
  println!("do_twice result is {}", result);
}

// As an example of where you could use either a closure defined inline or a named function, let’s
// look at a use of the map method provided by the Iterator trait in the standard library. To use the
// map function to turn a vector of numbers into a vector of strings, we could use a closure, like this:
fn closure_or_fnptr() {
  let list_of_numbers = vec![1, 2, 3];

  let list_of_strings_closure: Vec<String> =
    list_of_numbers.iter().map(|i| i.to_string()).collect();
  println!("{:?}", list_of_strings_closure);

  // Note that we must use the fully qualified syntax because there are multiple functions available named
  // to_string. Here, we’re using the to_string function defined in the ToString trait, which the standard
  // library has implemented for any type that implements Display (`i32` in this case).
  let list_of_strings_fnptr: Vec<String> =
    list_of_numbers.iter().map(ToString::to_string).collect();
  println!("{:?}", list_of_strings_fnptr);
}

// Recall from the “Enum values” that the name of each enum variant that we define also becomes an initializer
// function. We can use these initializer functions as function pointers that implement the closure traits,
// which means we can specify the initializer functions as arguments for methods that take closures, like so:
#[derive(Debug)]
enum Status {
  Value(u32),
  Stop,
}

fn enum_init_fnptr() {
  // Here we create Status::Value instances using each u32 value in the range that map is called on by using
  // the initializer function of Status::Value. Some prefer this style, and others prefer to use closures.
  // THEY COMPILE TO THE SAME CODE, so use whichever style is clearer to you.
  let list_of_statuses: Vec<Status> = (0u32..12).map(Status::Value).collect();
  println!("{:?}", list_of_statuses);
  if let Status::Value(x) = list_of_statuses[0] {
    println!("First status' value: {}", x)
  }
  println!("{:?}", Status::Stop);
}

// Function pointers implement all three of the closure traits (Fn, FnMut, and FnOnce), meaning you can always
// pass a function pointer as an argument for a function that expects a closure. It’s best to write functions
// using a generic type and one of the closure traits so your functions can accept either functions or closures.
// -----
// Consider implementing a register function that takes a callback in two ways:
//   fn register1(cb: fn(Event) -> ());
//   fn register2<F>(cb: F) where F: Fn(Event) -> ();
//
// Q: Which type signature permits register to take the widest variety of arguments?
// A: Closures with environments can be passed to register2, while only top-level functions
//    (or closures without environments) can be passed to register1.

pub fn run() {
  func_ptr();
  closure_or_fnptr();
  enum_init_fnptr();
}
