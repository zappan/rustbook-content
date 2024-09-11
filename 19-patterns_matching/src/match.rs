pub fn run() {
  let x = Some(2);
  match x {
    None => None,
    Some(i) => Some(i + 1),
  };

  return;
}
