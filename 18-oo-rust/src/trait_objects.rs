use gui::{Button, Draw, Screen};
use std::thread;
use std::time::Duration;

struct SelectBox {
  width: u16,
  height: u16,
  options: Vec<String>,
}

impl Draw for SelectBox {
  fn draw(&self) {
    println!("Drawing a select box");
  }
}

pub fn run() {
  let screen = Screen::new(vec![
    Box::new(Button {
      width: 16,
      height: 8,
      label: String::from("Hello button"),
    }),
    Box::new(SelectBox {
      width: 16,
      height: 12,
      options: vec![],
    }),
  ]);

  println!("Drawing screen...");
  screen.draw();
  println!("Screen drawn\n");

  thread::sleep(Duration::from_millis(320));

  println!("Rerawing screen...");
  screen.draw();
  println!("Screen redrawn");
}
