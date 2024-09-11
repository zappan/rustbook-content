// Unlike with tuples, in a struct you’ll name each piece of data so
// it’s clear what the values mean. Adding these names means that
// structs are more flexible than tuples: you don’t have to rely on the
// order of the data to specify or access the values of an instance.

struct User {
  username: String,
  email: String,
  is_active: bool,
  sign_in_count: u64,
}

// Tuple structs are useful when you want to give the whole tuple a name
// and make the tuple a different type from other tuples, and when naming
// each field as in a regular struct would be verbose or redundant.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like structs are structs that don't have any fields, and behave
// similarly to unit-type tuple '()'. Useful when you need to implement
// a trait on some type but don't have any data you want to store in it.
struct AlwaysEqual;

fn build_user(email: String, username: String) -> User {
  // using the field init shorthand syntax for email and username
  User {
    email,
    username,
    is_active: true,
    sign_in_count: 1,
  }
}

fn struct_type() {
  let mut user1 = build_user(
    String::from("someusername123"),
    String::from("someone@example.com"),
  );

  // NOTE: we need to println before creating user2, as borrowing error occurs on the
  // email property which will have been owned by user2 if println after user2 creation
  println!(
    "{} {} {} {}",
    user1.username, user1.email, user1.is_active, user1.sign_in_count
  );

  // struct update syntax
  let user2 = User {
    username: String::from("anotherusername123"),
    ..user1 // remaining filelds should have the same value as the fields in user1
  };

  // NOTE: here, we can no longer use user1 after creating user2 because the String
  // in the email field of user1 was moved into user2. If we had given user2 new
  // String values for both email and username, and thus only used the active and
  // sign_in_count values from user1, then user1 would still be valid after creating user2.

  println!(
    "{} {} {} {}",
    user2.username, user2.email, user2.is_active, user2.sign_in_count
  );

  user1.email = String::from("another.email@example.com");
  println!("User1's new email is: {}", user1.email);
}

fn tuple_struct_type() {
  let color: Color = Color(12, 33, 251);
  let point: Point = Point(32, 66, 424);
  println!("RGB Color => {}:{}:{}", color.0, color.1, color.2);
  println!("Point X:Y:rad => {}:{}:{}", point.0, point.1, point.2);
}

fn unit_like_struct_type() {
  // Traits will be discussed later, where this construct is used.. intentional(ly) unused for now
  let _subject = AlwaysEqual;
}

pub fn structs() {
  struct_type();
  tuple_struct_type();
  unit_like_struct_type();
}
