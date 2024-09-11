use std::cmp::PartialOrd; // denotes a 'comparable' type (comparable using '>' in our case)

fn largest<T: PartialOrd>(list: &[T]) -> &T {
  let mut largest = &list[0];
  for item in list {
    if *item > *largest {
      largest = item
    }
  }
  largest
}

pub fn main() {
  let num_list = vec![14, 32, 101, 43, 22];
  let largest_num = largest(&num_list);
  println!("The largest number in the list is: {}", largest_num);

  let char_list = vec!['c', 'z', 'x', 'y', 'w'];
  let largest_char = largest(&char_list);
  println!("The largest char in list is: {}", largest_char);
}
