mod enum_basics;
mod enum_match;
mod if_let_match;
mod option_t_match;

fn spacer() {
  println!("--------------------");
}

fn main() {
  spacer();

  enum_basics::option_enum_basics();
  spacer();

  enum_match::enum_match();
  spacer();

  option_t_match::option_t_match();
  spacer();

  if_let_match::if_let();
}
