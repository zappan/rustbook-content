#[no_mangle]
pub extern "C" fn call_from_c() {
  println!("Just called a Rust function from C!");
}

extern "C" {
  fn abs(input: i32) -> i32;
}

pub fn run() {
  println!("\nEXTERNAL CODE:");

  unsafe {
    println!("Absolute value of -3 according to C: {}", abs(-3));
  }
}
