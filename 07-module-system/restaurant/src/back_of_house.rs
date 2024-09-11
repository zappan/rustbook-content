pub struct _Breakfast {
  pub _toast: String,
  _seasonal_fruit: String,
}

impl _Breakfast {
  // because back_of_house::_Breakfast has a private field, the struct needs to
  // provide a public associated function that constructs an instance of Breakfast
  pub fn _summer(toast: &str) -> _Breakfast {
    _Breakfast {
      _toast: String::from(toast),
      _seasonal_fruit: String::from("peaches"),
    }
  }
}

#[derive(Debug)]
pub enum _Appetizer {
  _Soup,
  _Salad,
}

fn _cook_order() {}

fn _fix_incorrect_order() {
  _cook_order();
  super::_deliver_order(); // doesn't need to be public as it's an ancestor
}
