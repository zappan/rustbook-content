mod boxes;
mod deref_trait;
mod drop_trait;
mod ref_counted;
mod ref_cycles_memleaks;
mod ref_mutable;

fn main() {
  println!("Hello, world!");
  boxes::run();
  println!("------------------------------");
  deref_trait::run();
  println!("------------------------------");
  drop_trait::run();
  println!("------------------------------");
  ref_counted::run();
  println!("------------------------------");
  ref_mutable::run();
  println!("------------------------------");
  ref_cycles_memleaks::run();
}
