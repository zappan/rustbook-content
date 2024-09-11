use std::thread;
use std::time::Duration;

// Note that when the main thread of a Rust program completes, all spawned threads
// are shut down, whether or not they have finished running.
pub fn non_waiting_threads() {
  println!("--- Non-waiting threads ---");

  // calls to thread::sleep force a thread to stop its execution
  // for a short duration, allowing a different thread to run.
  thread::spawn(|| {
    for i in 1..10 {
      println!("Hi, number {i} from the spawned thread");
      thread::sleep(Duration::from_millis(24));
    }
  });

  for i in 1..5 {
    println!("Hi, number {i} from the main thread");
    thread::sleep(Duration::from_millis(24));
  }

  println!("---------------------------");
}

fn waiting_threads() {
  println!("--- Waiting threads ---");

  // calls to thread::sleep force a thread to stop its execution
  // for a short duration, allowing a different thread to run.
  let thread_handle = thread::spawn(|| {
    for i in 1..10 {
      println!("Hi, number {i} from the spawned thread");
      thread::sleep(Duration::from_millis(24));
    }
  });

  for i in 1..5 {
    println!("Hi, number {i} from the main thread");
    thread::sleep(Duration::from_millis(24));
  }

  thread_handle.join().unwrap(); // blocks the currently running thread till the spawned one completes
  println!("---------------------------");
}

// By adding the move keyword before the closure, we force the closure to take ownership
// of the values it’s using rather than allowing Rust to infer that it should borrow the values.
fn move_closures() {
  let v = vec![2, 3, 4, 5];

  // `move || ` forces the closure to take ownership of variables it uses, in this case `v`
  let handle = thread::spawn(move || {
    println!("Here's a vector {v:?}");
  });

  // # without `move ||` in the closure, this would be possible...
  // drop(v); // Oh, no!

  handle.join().unwrap();
}

pub fn run() {
  waiting_threads();
  move_closures();

  // --------- needs to be called last to prove the idea ----------- //
  // ## we'll call this from the main.rs
  // non_waiting_threads();
}
