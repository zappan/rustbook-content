mod messaging;
mod shared_state;
mod sync_and_send_traits;
mod threads;

fn main() {
  threads::run();
  println!("--------------------");
  messaging::run();
  println!("--------------------");
  shared_state::run();
  println!("--------------------");
  sync_and_send_traits::run();
  println!("--------------------");

  // --------- needs to be called last to prove the idea ----------- //
  // ## calling this from the main.rs
  threads::non_waiting_threads();
}
