use super::back_of_house;

// ## the previous 'hosting' path import is out of scope within the customer
// ## module, so it needs to be re-referenced from within the module with
// ## either a direct import, or referencing the shortcut in the parent module
// use super::hosting; // parent shortcut, if we want to use that
use crate::front_of_house::hosting; // direct import

pub fn _eat_at_restaurant() {
  crate::front_of_house::hosting::_add_to_waitlist(); // absolute path
  super::front_of_house::hosting::_add_to_waitlist(); // relative path
  hosting::_add_to_waitlist(); // 'use'-imported path
}

pub fn _have_breakfast() {
  // order a breakfast in the summer with Rye toast:
  let mut meal = back_of_house::_Breakfast::_summer("Rye");
  // change our mind about what bread I'd like
  meal._toast = String::from("Wheat");
  println!("I'd like {} toast please", meal._toast);

  // ## The next line won't compile if we uncomment it; we're not allowed
  // ## to see or modify the seasonal fruit that comes with the meal
  // meal.seasonal_fruit = String::from("blueberries");
}
pub fn _have_an_appetizer() {
  let order1 = back_of_house::_Appetizer::_Salad;
  let order2 = back_of_house::_Appetizer::_Soup;
  println!("Order 1 was {:?}", order1);
  println!("Order 2 was {:?}", order2);

  // --------------------------------------------------------------------------
}
