use aggregator::Summary;

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}: {}", self.authored_by(), self.content)
  }

  fn authored_by(&self) -> String {
    format!("@{}", self.username)
  }
}
