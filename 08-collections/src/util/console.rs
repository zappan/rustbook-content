use std::str;

pub fn section(name: &str) {
  // let's use shadowing here, as I just learned about that concept
  let delim = vec![b'-'; name.chars().count()];
  let delim = str::from_utf8(&delim).unwrap();

  println!();
  println!("{}", name);
  println!("{}", delim);
}

pub fn spacer() {
  println!("---------------");
}
