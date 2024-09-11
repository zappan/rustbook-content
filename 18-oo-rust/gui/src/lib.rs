pub trait Draw {
  fn draw(&self);
}

pub struct Screen {
  pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
  pub fn draw(&self) {
    for c in self.components.iter() {
      c.draw();
    }
  }

  pub fn new(components: Vec<Box<dyn Draw>>) -> Self {
    Self { components }
  }
}

pub struct Button {
  pub width: u16,
  pub height: u16,
  pub label: String,
}

impl Draw for Button {
  fn draw(&self) {
    println!("Drawing a button...");
  }
}
