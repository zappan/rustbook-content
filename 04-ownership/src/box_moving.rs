// fn add_suffix(name: String) -> String {
fn add_suffix(mut name: String) -> String {
  name.push_str(" Jr.");
  name
}

// If you move a variable, Rust will stop you from using that variable later.
// More generally, the compiler will enforce this principle:
// > Moved heap data principle: if a variable x moves ownership of heap data
// > to another variable y, then x cannot be used after the move.

pub fn ownership_transfer() {
  let first = String::from("Ferris");
  let full = add_suffix(first);
  println!("{full}");

  // println!("{first}"); // first is now used here; this value has been moved, compiler errors with "value borrowed here after move"

  // println!("{full}, originally {first}"); // first is now used here; same error
}
