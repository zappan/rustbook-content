use blog_nonrust_way::Post;

// Notice that the only type we’re interacting with from the crate is the Post type. This
// type will use the state pattern and will hold a value that will be one of three state
// objects  representing the various states a post can be in—draft, waiting for review, or
// published. Changing from one state to another will be managed internally within the Post type.
// ----
// The states change in response to the methods called by our library’s users on the Post
// instance, but they don’t have to manage the state changes directly. Also, users CAN’T
// make a mistake with the states, like publishing a post before it’s reviewed.
pub fn run() {
  let mut post = Post::new();
  assert_eq!("", post.content());
  println!("Blog post content after creation:\n{}", post.content());

  post.add_text("I ate a salad for lunch today");
  assert_eq!("", post.content());
  println!("Blog post content after adding text:\n{}", post.content());

  // users CAN’T make a mistake with the states, like publishing a post before it’s reviewed.
  post.approve();
  assert_eq!("", post.content());
  println!("Blog post content after approval:\n{}", post.content());

  // only here, we're asking for a review
  post.request_review();
  assert_eq!("", post.content());
  println!(
    "Blog post content after review request:\n{}",
    post.content()
  );

  post.approve();
  assert_eq!("I ate a salad for lunch today", post.content());
  println!("Blog post content after approval:\n{}", post.content());
}
