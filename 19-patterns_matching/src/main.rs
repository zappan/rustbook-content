mod at_bindings;
mod fn_params_pattern;
mod for_loops;
mod if_let;
mod let_pattern;
mod r#match;
mod match_guards;
mod refutability;
mod while_let_loops;

fn main() {
  println!("Hello, world!");

  r#match::run();
  if_let::run();
  while_let_loops::run();
  for_loops::run();
  let_pattern::run();
  fn_params_pattern::run();
  println!("--------------------");
  refutability::run();
  println!("--------------------");
  match_guards::run();
  println!("--------------------");
  at_bindings::run();
}
