fn lazy_iterators() {
  // In Rust, iterators are lazy, meaning they have no effect until you call methods that
  // consume the iterator to use it up. This code by itself doesn’t do anything useful.
  let v1 = vec![1, 2, 3];
  let v1_iter = v1.iter();

  // only when we consume the iterator, something happens:
  for val in v1_iter {
    print!("{val} ");
  }
  println!();
}

// Iterator adaptors are methods defined on the Iterator trait that don’t consume the iterator.
// Instead, they produce different iterators by changing some aspect of the original iterator.
fn iterator_adaptors() {
  let v1 = vec![1, 2, 3];
  // v1.iter().map(|x| x + 1); // causes lazy iterator warning

  let mut res = v1.iter().map(|x| x + 1);
  while let Some(item) = res.next() {
    print!("{item} ");
  }
  println!();

  let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
  println!("{:?}", v2);
}

#[derive(Debug)]
struct Shoe {
  size: u32,
  style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
  shoes.into_iter().filter(|s| s.size == size).collect()
}

fn env_capturing_iterators() {
  let shoes = vec![
    Shoe {
      size: 10,
      style: String::from("boot"),
    },
    Shoe {
      size: 12,
      style: String::from("sneaker"),
    },
    Shoe {
      size: 13,
      style: String::from("sandal"),
    },
    Shoe {
      size: 10,
      style: String::from("sneaker"),
    },
  ];

  println!("All shoes: {:?}", shoes);
  let my_fit_shoes = shoes_in_my_size(shoes, 10);
  println!("Shoes fitting me: {:?}", my_fit_shoes);
}

fn main() {
  lazy_iterators();
  iterator_adaptors();
  println!("--------------------------------");
  env_capturing_iterators();
}
