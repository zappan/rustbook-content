unsafe fn dangerous() {
  println!("Hello from unsafe function");
}

pub fn run() {
  println!("\nUNSAFE FUNCTIONS:");

  unsafe {
    dangerous();
  }
}
