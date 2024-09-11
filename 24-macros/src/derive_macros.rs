use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

#[derive(HelloMacro)]
struct Ananas;

pub fn run() {
  Pancakes::hello_macro();
  Ananas::hello_macro();
}
