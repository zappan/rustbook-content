use crate::console;

// The type HashMap<K, V> stores a mapping of keys of type K to values of type V using
// a hashing function, which determines how it places these keys and values into memory.

// This kind of data structure in other programming languages is often called differently:
// hash, map, object, hash table, dictionary, or associative array, just to name a few.

// As always, check the standard library documentation for more information, as many more
// goodies are hiding in the functions defined on HashMap<K, V> by the standard library.

use std::collections::HashMap;

// Just like vectors:
// -- hash maps store their data on the heap.
// -- hash maps are homogeneous: all of the keys must have the same type as each other,
//    and all of the values must have the same type.
pub fn hashmaps() {
  console::section("HASHMAPS:");

  hashmap_creation();
  hashmap_values_access();
  iterate_over_hashmap();
  hashmap_and_ownership();

  console::spacer();
  hashmap_updates();

  console::spacer();
  hashmap_challenge();
}

fn hashmap_creation() {
  // This HashMap has keys of type String and values of type i32.
  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);
  println!("{:?}", scores);
}

fn hashmap_updates() {
  // overwriting a value
  let mut scores = HashMap::new();
  let team_name = String::from("Blue");
  scores.insert(&team_name, 10);
  scores.insert(&team_name, 25);
  println!("{:?}", scores);

  // keeping the old value, disregarding the new
  // aka adding only if key is not present -> entry().or_insert()
  let team_blue = String::from("Blue");
  let team_yellow = String::from("Yellow");
  let mut scores = HashMap::new();
  scores.insert(&team_blue, 10);

  // or_insert() returns a mutable reference to the old value or the inserted value
  let yellow_score = *scores.entry(&team_yellow).or_insert(50);
  let blue_score = *scores.entry(&team_blue).or_insert(25);
  println!("{:?}", scores);
  println!("Blue score: {blue_score}; Yellow score: {yellow_score}",);

  // updating the value based on the old value; example: word-counter
  let text = "Hello world you wonderful world";
  let mut word_counter = HashMap::new();
  for word in text.split_whitespace() {
    let count = word_counter.entry(word).or_insert(0);
    *count += 1;
  }
  println!("Word count is: {:?}", word_counter);
}

fn hashmap_values_access() {
  // ...by providing its key to the get() method
  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  // get() returns an Option<&V> of HashMap<K, V> for type safety...
  let team_name = String::from("Blue");
  let team_score = scores.get(&team_name); // borrow so it doesn't destroy the string
  if let Some(score) = team_score {
    println!("{team_name} team's score is {score}",);
  }

  // alternatively we can unwrap with a fallback value:
  // aka handle the Option by calling copied() to get an Option<i32> rather than an Option<&i32>,
  // then unwrap_or() to set score to zero if scores doesn't have an entry for the key.
  let team_name = String::from("Blue");
  let team_score = scores.get(&team_name).copied().unwrap_or(0);
  println!("{team_name} team's score is {team_score}",);

  let team_name = String::from("Brown");
  let team_score = scores.get(&team_name).copied().unwrap_or(0);
  println!("{team_name} team's score is {team_score}",);
}

// iterate over each key/value pair in a hash map in a similar manner as with vectors
fn iterate_over_hashmap() {
  let mut scores = HashMap::new();
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  // the order if arbitrary (hashmaps are not ordered)
  for (key, value) in scores {
    println!("{key}: {value}",);
  }
}

// HASHMAP TAKES OWNERSHIP:
// -------------------------
// For types that implement the Copy trait, like i32, the values are copied into the hash map.
// For owned values like String, the values will be moved into the hash map
// ---------------------------------------------------------------------------------------------
// If we insert references to values into the hash map, the values won’t be moved into the hash map.
// The values the references point to must be valid for at least as long as the hash map is valid.
fn hashmap_and_ownership() {
  let field_name = String::from("Favourite color");
  let field_value = String::from("Blue");

  let mut map = HashMap::new();
  map.insert(field_name, field_value); // you may .clone() those strings if you wish to keep them

  // ## field_name and field_value are invalid at this point
  // ## try using them and see what compiler error you get!
  // println!("{:?} {:?}", field_name, field_value);

  let field_name = String::from("Favourite color");
  let field_value = String::from("Blue");

  let mut map = HashMap::new();
  map.insert(field_name.clone(), field_value.clone()); // you may .clone() those strings if you wish to keep them
  println!("{field_name}: {field_value}");
}

fn hashmap_challenge() {
  let mut map: HashMap<char, Vec<usize>> = HashMap::new();
  for (idx, c) in "hello".chars().enumerate() {
    map.entry(c).or_insert(Vec::new()).push(idx); // append the index to the list of indices
  }
  let mut sum = 0;
  for i in map.get(&'l').unwrap() {
    sum += *i; // works without a pointer as well...
  }
  println!("In the {:?}, the sum of indices of 'l' is: {}", map, sum);
}
