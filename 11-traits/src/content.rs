mod news_articles;
mod tweets;

use aggregator::Summary;
pub use news_articles::NewsArticle;
pub use tweets::Tweet;

// implTrait
pub fn notify_impl(item: &impl Summary) {
  println!("Breaking news: {}", item.summarize());
}
// Trait Bound Syntax: The impl Trait syntax works for straightforward cases but is
// actually syntax sugar for a longer form known as a trait bound; it looks like this:
pub fn notify_trait_bound<T: Summary>(item: &T) {
  println!("Breaking news: {}", item.summarize());
}

// This longer form is equivalent to the example in the previous section
// but is more verbose. We place trait bounds with the declaration of the
// generic type parameter after a colon and inside angle brackets.
// -------------
// The impl Trait syntax is convenient and makes for more concise code in simple cases,
// while the fuller trait bound syntax can express more complexity in other cases.
// For example, we can have two parameters that implement Summary.
// -------------
// Doing so with the impl Trait syntax looks like this:
// > pub fn notify(item1: &impl Summary, item2: &impl Summary) {..}
// Using impl Trait is appropriate if we want this function to allow item1 and item2
// to have different types (as long as both types implement Summary).
// -------------
// If we want to force both parameters to have the same type, however,
// we MUST use a trait bound, like this:
// > pub fn notify<T: Summary>(item1: &T, item2: &T) {..}
// The generic type T specified as the type of the item1 and item2 parameters
// constrains the function such that the concrete type of the value passed as
// an argument for item1 and item2 must be the same.

// MULTIPLE TRAIT BOUNDS WITH THE + SYNTAX
// We can also specify more than one trait bound. Say we wanted notify to use display
// formatting as well as summarize on item: we specify in the notify definition that
// item must implement both Display and Summary. We can do so using the + syntax:
// > pub fn notify(item1: &(impl Summary + Display)) {..}
// > pub fn notify<T: Summary + Display>(item1: &T) {..}
// With these two, the body of notify can call summarize and use {} to format item.

// CLEARER TRAIT BOUNDS WITH WHERE CLAUSES
// Using too many trait bounds has its downsides. Each generic has its own trait bounds,
// so functions with multiple generic type parameters can contain lots of trait bound
// information between the function’s name and its parameter list, making the function
// signature hard to read. For this reason, Rust has alternate syntax for specifying
// trait bounds inside a where clause after the function signature.
// So instead of writing this:
// > fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {..}
// we can use a where clause, like this:
// > fn some_function<T, U>(t: &T, u: &U) -> i32
// > where
// >     T: Display + Clone,
// >     U: Clone + Debug,
// > { .. }

// We can also use the impl Trait syntax in the return position to return a value
// of some type that implements a trait, as shown here:
pub fn returns_summarizable() -> impl Summary {
  // Returning either a NewsArticle or a Tweet from here isn’t allowed due to
  // restrictions around how the impl Trait syntax is implemented in the compiler.
  // We’ll cover how to write a function with this behavior in the “Using Trait
  // Objects That Allow for Values of Different Types” section of Chapter 17.
  Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  }
}
