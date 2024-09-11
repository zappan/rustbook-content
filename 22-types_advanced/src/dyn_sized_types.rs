// https://rust-book.cs.brown.edu/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait
// ------------------------------------------------------------------------------------------------------

// Rust implicitly adds a bound on Sized to every generic function. That is, a generic function definition like this:
//
//   fn generic<T>(t: T) {
//     // --snip--
//   }
//
// is actually treated as though we had written this:
//
//   fn generic<T: Sized>(t: T) {
//     // --snip--
//   }
//
// By default, generic functions will work only on types that have a known size at compile time.
// However, you can use the following special syntax to relax this restriction:
//
//   fn generic<T: ?Sized>(t: &T) {
//     // --snip--
//   }
//
// A trait bound on ?Sized means “T may or may not be Sized” and this notation overrides the default
// that generic types must have a known size at compile time. The ?Trait syntax with this meaning is
// only available for Sized, not any other traits. Also note that we switched the type of the t parameter
// from T to &T. Because the type might not be Sized, we need to use it behind some kind of pointer.
// In this case, we’ve chosen a reference.
