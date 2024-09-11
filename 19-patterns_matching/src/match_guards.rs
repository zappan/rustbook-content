// A match guard is an additional if condition, specified after the pattern
// in a match arm, that must also match for that arm to be chosen.
// -------------------------------------------------------------------------------
// The downside of this additional expressiveness is that arms with match guards
// don’t “count” towards exhaustiveness. So even if we added Some(x) if x % 2 == 1
// as an additional arm, we would still need the un-guarded Some(x) arm.
fn match_guard(num: Option<i32>) {
  match num {
    Some(x) if x % 2 == 0 => println!("The number {x} is even"),
    Some(x) => println!("The number {x} is odd"),
    None => (),
  }
}

fn another_match_guard() {
  let x = Some(5);
  let y = 10;

  // The pattern in the second match arm doesn’t introduce a new variable `y` that would
  // shadow the outer y, meaning we can use the outer y in the match guard. Instead of
  // specifying the pattern as Some(y), which would have shadowed the outer y, we specify Some(n).
  match x {
    Some(50) => println!("Got 50"),
    Some(n) if n == y => println!("Matched, n = {n}"),
    _ => println!("Default case, x = {x:?}"),
  }

  println!("at the end: x = {x:?}, y = {y}");
}

pub fn run() {
  match_guard(Some(4));
  match_guard(Some(3));
  match_guard(None);
  another_match_guard();
}
