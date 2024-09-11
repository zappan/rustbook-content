// Another method of handling concurrency would be for multiple threads to access
// the same shared data. Consider this part of the slogan from the Go language
// documentation again: “do not communicate by sharing memory.”
// ------------------------------------------------------------------------------
// In a way, channels in any programming language are similar to single ownership,
// because once you transfer a value down a channel, you should no longer use that
// value. Shared memory concurrency is like multiple ownership: multiple threads
// can access the same memory location at the same time.

// A mutex allows only one thread to access some data at any given time, guarding
// data it holds via the locking system.
// To use data:
//   * You must attempt to acquire the lock before using the data.
//   * When you’re done with the data that the mutex guards, you must
//     unlock the data so other threads can acquire the lock.

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn mutex_communication() {
  let m = Mutex::new(4);
  println!("m = {m:?}");

  {
    // Mutex<T> is a smart pointer. More accurately, the call to lock returns a smart pointer
    // called MutexGuard, wrapped in a LockResult that we handled with the call to unwrap().
    let mut num = m.lock().unwrap();
    *num = 7;
  }
  // The smart pointer also has a Drop implementation that releases the lock automatically
  // when a MutexGuard goes out of scope, which is after the above closing curly brace.

  println!("m = {m:?}");
}

// Using an Arc<T> to wrap the Mutex<T> to be able to share ownership across multiple threads
// ------------------------------------------------------------------------------------------
// An Arc is STILL NOT SAFE to use if its data contains a reference (like a 'String' type), since
// the reference could (in theory) be invalidated before all threads containing the Arc finish executing.
// ------------------------------------------------------------------------------------------
fn shared_mutex() {
  let mut thread_handles = vec![];
  // Arc<T> is a type like Rc<T> that is safe to use in concurrent situations.
  // The a stands for atomic, meaning it’s an atomically reference counted type.
  let mutex_counter = Arc::new(Mutex::new(0));

  for _ in 0..10 {
    let mutex_counter = Arc::clone(&mutex_counter);
    let handle = thread::spawn(move || {
      let mut count = mutex_counter.lock().unwrap();
      *count += 1;
    }); // When a thread finishes running its closure, count will go out of scope
        // and release the lock so another thread can acquire it.
    thread_handles.push(handle);
  }

  for handle in thread_handles {
    handle.join().unwrap();
  }

  println!("Mutex Result: {}", *mutex_counter.lock().unwrap());
}

pub fn run() {
  mutex_communication();
  println!("---------------------");
  shared_mutex();
  println!("---------------------");
  // println!("---------------------");
}
