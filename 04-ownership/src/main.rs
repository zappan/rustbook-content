mod box_borrowing;
mod box_cloning;
mod box_moving;

// Summary
// Ownership is primarily a discipline of heap management:
//   All heap data must be owned by exactly one variable.
//   Rust deallocates heap data once its owner goes out of scope.
//   Ownership can be transferred by moves, which happen on assignments and function calls.
//   Heap data can only be accessed through its current owner, not a previous owner.
// ----------------------------------------------------------------------------------------
// Pointers can be created either through boxes (pointers owning data on the heap)
// or references (non-owning pointers). See box_borrowing for examples
fn main() {
  box_moving::ownership_transfer();
  println!();
  box_cloning::ownership_transfer();
  println!();
  box_borrowing::ownership_borrowing();
}
