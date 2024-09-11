// --------------------------------------------------------------------------------------------------------
// https://rust-book.cs.brown.edu/ch17-03-oo-design-patterns.html
// --------------------------------------------------------------------------------------------------------
// The implementation using the state pattern is easy to extend to add more functionality.
// To see the simplicity of maintaining code that uses the state pattern, try a few of these suggestions:
//  * Add a reject method that changes the post’s state from PendingReview back to Draft.
//  * Require two calls to approve before the state can be changed to Published.
//  * Allow users to add text content only when a post is in the Draft state. Hint: have the state object
//    responsible for what might change about the content but not responsible for modifying the Post.

// pub trait State {
//   // these intentionally consume state and return a new state...
//   fn request_review(self: Box<Self>) -> Box<dyn State>;
//   fn approve(self: Box<Self>) -> Box<dyn State>;

//   // A default implementation for the content method that returns an empty string slice,
//   // meaning we don’t need to implement content on the Draft and PendingReview structs.
//   fn content<'a>(&self, _post: &'a Post) -> &'a str {
//     ""
//   }
// }

// struct Draft {}
// struct PendingReview {}
// struct Published {}

// impl State for Draft {
//   fn request_review(self: Box<Self>) -> Box<dyn State> {
//     Box::new(PendingReview {})
//   }

//   fn approve(self: Box<Self>) -> Box<dyn State> {
//     self
//   }
// }

// impl State for PendingReview {
//   fn request_review(self: Box<Self>) -> Box<dyn State> {
//     self
//   }

//   fn approve(self: Box<Self>) -> Box<dyn State> {
//     Box::new(Published {})
//   }
// }

// impl State for Published {
//   fn request_review(self: Box<Self>) -> Box<dyn State> {
//     self
//   }

//   fn approve(self: Box<Self>) -> Box<dyn State> {
//     self
//   }

//   fn content<'a>(&self, post: &'a Post) -> &'a str {
//     &post.content
//   }
// }

// ==================================================

// The structs no longer have the state field because we’re moving the encoding
// of the state to the types of the structs. The Post struct will represent a
// published post, and it has a content method that returns the content.

// Implementing Transitions as Transformations into Different Types:
// -----------------------------------------------------------------
// So how do we get a published post? We want to enforce the rule that a draft post has to
// be reviewed and approved before it can be published. A post in the pending review state
// should still not display any content. Let’s implement these constraints by adding another
// struct, PendingReviewPost, defining the request_review method on DraftPost to return a
// PendingReviewPost, and defining an approve method on PendingReviewPost to return a Post.

pub struct Post {
  content: String,
}

pub struct PendingReviewPost {
  content: String,
}

pub struct DraftPost {
  content: String,
}

impl Post {
  pub fn new() -> DraftPost {
    DraftPost {
      content: String::new(),
    }
  }

  pub fn content(&self) -> &str {
    &self.content
  }
}

impl PendingReviewPost {
  // we want to consume PendingReviewPost and create and return a "published" Post
  pub fn approve(self) -> Post {
    Post {
      content: self.content,
    }
  }
}

impl DraftPost {
  pub fn add_text(&mut self, text: &str) {
    // self.content = text.to_string();
    self.content.push_str(text);
  }

  // we want to consume DraftPost and create and return a PendingReviewPost
  pub fn request_review(self) -> PendingReviewPost {
    PendingReviewPost {
      content: self.content,
    }
  }
}
