use blog_rust_way::Post;

// Encoding States and Behavior as Types
// ---------------------------------------
// We’ll show you how to rethink the state pattern to get a different set of trade-offs.
// Rather than encapsulating the states and transitions completely so outside code has no
// knowledge of them, we’ll encode the states into different types. Consequently, Rust’s
// type checking system will prevent attempts to use draft posts where only published posts
// are allowed by issuing a compiler error.

// ---------------------------------------
// The changes we needed to make to main to reassign post mean that this implementation
// DOESN’T quite follow the object-oriented state pattern anymore: the transformations between
// the states are no longer encapsulated entirely within the Post implementation. HOWEVER,
// our gain is that invalid states are now impossible because of the type system and the type
// checking that happens at compile time! This ensures that certain bugs, such as display of
// the content of an unpublished post, will be discovered before they make it to production.
pub fn run() {
  let mut post = Post::new();

  // ## Compiler Error: method not available on the DraftPost
  // assert_eq!("", post.content());
  // println!("Blog post content after creation:\n{}", post.content());

  post.add_text("I ate a salad for lunch today");

  // ## Compiler Error: method not available on the DraftPost
  // assert_eq!("", post.content());

  // ## Compiler Error: method not available on the DraftPost
  // ## so that users CAN’T make a mistake with the states, like publishing a post before it’s reviewed.
  // post.approve();

  // ## first, we're only able to ask for a review -> PendingReviewPost
  let post = post.request_review();

  // ## Compiler Error: method not available on the PendingReviewPost
  // assert_eq!("", post.content());

  // ## only then we're able to approve/publish the post -> Post
  let post = post.approve();

  // ## And, finally, we have the content() method available to get the content out
  assert_eq!("I ate a salad for lunch today", post.content());
  println!("Blog post content after approval:\n{}", post.content());
}
