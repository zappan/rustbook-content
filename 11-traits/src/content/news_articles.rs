use aggregator::Summary;

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!(
      "{}, by {} ({})",
      self.headline,
      self.authored_by(),
      self.location
    )
  }

  fn authored_by(&self) -> String {
    format!("{}", self.author)
  }
}
