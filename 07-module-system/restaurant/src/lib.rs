mod back_of_house;
mod customer;
mod front_of_house;

// use crate::front_of_house::hosting;
pub use crate::front_of_house::hosting; // re-exporting the name for external uset o enable
                                        // the code that calls our code to refer to that name
                                        // as if it had been defined in that code’s scope

pub use customer::*;

// used from back_of_house
fn _deliver_order() {}
