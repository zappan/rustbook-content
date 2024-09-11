mod compounds;
mod enums;
mod scalars;
mod slices;
mod structs;

use crate::util::console;

// There are two data type subsets: SCALAR and COMPOUND
// SCALAR types: integers, floats, booleans and characters
// COMPOUND types: tuple, array,
pub fn datatypes() {
  console::spacer();

  console::section("DATA TYPES - SCALAR:");
  scalars::scalars();

  console::section("DATA TYPES - COMPOUND:");
  compounds::compounds();

  console::section("DATA TYPES - SLICE:");
  slices::slices();

  console::section("DATA TYPES - STRUCT:");
  structs::structs();

  console::section("DATA TYPES - ENUM:");
  enums::enums();
}
