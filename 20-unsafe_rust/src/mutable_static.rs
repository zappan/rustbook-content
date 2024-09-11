// Accessing and modifying mutable static variables is unsafe.
// With mutable data that is globally accessible, it’s difficult to ensure there are
// no data races, which is why Rust considers mutable static variables to be unsafe.
// ---
// Where possible, it’s preferable to use the concurrency techniques and thread-safe smart
// pointers so the compiler checks that data accessed from different threads is done safely.

static mut COUNTER: u32 = 0;

// this is also a "safe abstraction" over an unsafe code
fn add_to_count(inc: u32) {
  unsafe {
    COUNTER += inc;
  }
}

// same for this one, safe abstraction for accessing the static variable
pub fn run() {
  println!("\nMUTATING STATIC VARIABLES:");

  add_to_count(3);

  unsafe {
    println!("COUNTER: {COUNTER}");
  }
}
