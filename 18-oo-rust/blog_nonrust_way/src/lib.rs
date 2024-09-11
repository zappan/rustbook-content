// --------------------------------------------------------------------------------------------------------
// https://rust-book.cs.brown.edu/ch17-03-oo-design-patterns.html
// --------------------------------------------------------------------------------------------------------
// The implementation using the state pattern is easy to extend to add more functionality.
// To see the simplicity of maintaining code that uses the state pattern, try a few of these suggestions:
//  * Add a reject method that changes the post’s state from PendingReview back to Draft.
//  * Require two calls to approve before the state can be changed to Published.
//  * Allow users to add text content only when a post is in the Draft state. Hint: have the state object
//    responsible for what might change about the content but not responsible for modifying the Post.

pub trait State {
  // these intentionally consume state and return a new state...
  fn request_review(self: Box<Self>) -> Box<dyn State>;
  fn approve(self: Box<Self>) -> Box<dyn State>;

  // A default implementation for the content method that returns an empty string slice,
  // meaning we don’t need to implement content on the Draft and PendingReview structs.
  fn content<'a>(&self, _post: &'a Post) -> &'a str {
    ""
  }
}

struct Draft {}
struct PendingReview {}
struct Published {}

impl State for Draft {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    Box::new(PendingReview {})
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }
}

impl State for PendingReview {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
    Box::new(Published {})
  }
}

impl State for Published {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn content<'a>(&self, post: &'a Post) -> &'a str {
    &post.content
  }
}

pub struct Post {
  state: Option<Box<dyn State>>,
  content: String,
}

impl Post {
  pub fn new() -> Post {
    Self {
      state: Some(Box::new(Draft {})),
      content: String::new(),
    }
  }

  pub fn content(&self) -> &str {
    self.state.as_ref().unwrap().content(self)
  }

  pub fn add_text(&mut self, text: &str) {
    // self.content = text.to_string();
    self.content.push_str(text);
  }

  pub fn request_review(&mut self) {
    // We call the take() method to take the `Some` value out of the `state` field and
    // leave a `None` in its place, because Rust doesn’t let us have unpopulated fields
    // in structs. -- That's why `state` is defined as `Option<T>`
    // This lets us move the `state` value out of `Post` rather than borrowing it.
    // Then we’ll set the post’s `state` value to the result of this operation.
    // -------------------------------------------------------------------------------------
    // !!! This ensures Post can’t use the old state value in any way (a separate reference)
    // after we’ve transformed it into a new state. !!!
    if let Some(s) = self.state.take() {
      self.state = Some(s.request_review());
    }
  }

  pub fn approve(&mut self) {
    if let Some(s) = self.state.take() {
      self.state = Some(s.approve());
    }
  }

  // pub fn state(&self) -> State ...?
}
