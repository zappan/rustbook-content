// Cloning Avoids Moves
// One way to avoid moving data is to clone it using the .clone() method.
// For example, we can fix the safety issue in the 'box_moving.rs' program with a clone:

fn add_suffix(mut name: String) -> String {
  name.push_str(" Jr.");
  name
}

pub fn ownership_transfer() {
  let first = String::from("Ferris");
  let first_clone: String = first.clone();
  let full = add_suffix(first_clone); // ownership transfer happens on the 'first_clone' this time

  println!("{full}, originally {first}"); // first is now used here; but has not moved as it was previously cloned
}
