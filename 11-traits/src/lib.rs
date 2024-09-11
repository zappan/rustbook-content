pub trait Summary {
  // you may provide an interface to implement
  fn authored_by(&self) -> String;

  // you may provide an interface with a default implementation too (overridable)
  fn summarize(&self) -> String {
    format!("(Read more from {}...)", self.authored_by())
  }
}
