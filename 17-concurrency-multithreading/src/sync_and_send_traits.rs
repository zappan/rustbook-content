// Rust language has very few concurrency features. Almost every concurrency feature we’ve
// talked about so far in this chapter has been part of the standard library, not the language.
// -------------------------------------------------------------------------------------------
// Two concurrency concepts are embedded in the language: the std::marker traits Sync and Send.

// * The smart pointer Rc<T> is also neither Send nor Sync, for reasons described above.
// * The RefCell<T> type (which we talked about in Chapter 15) and the family of related
//   Cell<T> types are Send (if T: Send), but they are not Sync. A RefCell can be sent
//   across a thread boundary, but not accessed concurrently because the implementation
//   of borrow checking that RefCell<T> does at runtime is not thread-safe.
// * The smart pointer Mutex<T> is Send and Sync, and can be used to share access with
//   multiple threads as you saw in the “Sharing a Mutex<T> Between Multiple Threads” section.
// * The type MutexGuard<'a, T> that is returned by Mutex::lock is Sync (if T: Sync) but
//   not Send. It is specifically not Send because some platforms mandate that mutexes are
//   unlocked by the same thread that locked them.

// Implementing Send and Sync Manually Is Unsafe
// Because types that are made up of Send and Sync traits are automatically also Send and Sync,
// we don’t have to implement those traits manually. As marker traits, they don’t even have any
// methods to implement. They’re just useful for enforcing invariants related to concurrency.

pub fn run() {
  // no examples here
  // see: https://rust-book.cs.brown.edu/ch16-04-extensible-concurrency-sync-and-send.html
}
