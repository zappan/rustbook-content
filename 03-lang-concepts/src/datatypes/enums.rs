use crate::util::console;

#[derive(Debug)]
enum IpAddrKind {
  V4,
  V6,
}

// "naive" approach, at least in Rust context
#[derive(Debug)]
struct IpAddrStruct {
  kind: IpAddrKind,
  address: String,
}

#[derive(Debug)]
enum IpAddrEnumBasic {
  V4(String),
  V6(String),
}

#[derive(Debug)]
enum IpAddrEnum {
  V4(u8, u8, u8, u8),
  V6(String),
}

#[derive(Debug)]
struct Ipv4Addr {
  address: (u8, u8, u8, u8),
}

#[derive(Debug)]
struct Ipv6Addr {
  address: String,
}

#[derive(Debug)]
enum IpAddr {
  V4(Ipv4Addr),
  V6(Ipv6Addr),
}

fn route(ip: IpAddrKind) {
  println!("Routing IP.{:?}", ip);
}

fn ip_addresses() {
  let ip_v4: IpAddrKind = IpAddrKind::V4;
  let ip_v6: IpAddrKind = IpAddrKind::V6;
  println!("{:?} {:?}", ip_v4, ip_v6);

  route(ip_v4);
  route(ip_v6);
  route(IpAddrKind::V4);
  route(IpAddrKind::V6);
  console::spacer();

  let home: IpAddrStruct = IpAddrStruct {
    address: String::from("127.0.0.1"),
    kind: IpAddrKind::V4,
  };
  let loopback: IpAddrStruct = IpAddrStruct {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
  };

  println!(
    "Home IP Address is {} on an IP {:?}",
    home.address, home.kind
  );
  println!(
    "Loopback IP Address is {} on an IP {:?}",
    loopback.address, loopback.kind
  );
  console::spacer();

  // another detail of how enums work: the name of each enum variant that we define
  // also becomes a function that constructs an instance of the enum. That is, IpAddr::V4()
  // is a function call that takes a String argument and returns an instance of the IpAddr type.
  let home: IpAddrEnumBasic = IpAddrEnumBasic::V4(String::from("127.0.0.1"));
  let loopback: IpAddrEnumBasic = IpAddrEnumBasic::V6(String::from("::1"));
  println!("Home IP Address is {:?}", home);
  println!("Loopback IP Address is {:?}", loopback);
  console::spacer();

  // each variant can have different types and amounts of associated data.
  // Version four IP addresses will always have four numeric components that
  // will have values between 0 and 255. If we wanted to store V4 addresses
  // as four u8 values but still express V6 addresses as one String value,
  // we wouldn’t be able to with a struct. Enums handle this case with ease:
  let home: IpAddrEnum = IpAddrEnum::V4(127, 0, 0, 1);
  let loopback: IpAddrEnum = IpAddrEnum::V6(String::from("::1"));
  println!("Home IP Address is {:?}", home);
  println!("Loopback IP Address is {:?}", loopback);
  console::spacer();

  // each variant can have different types and amounts of associated data.
  // Version four IP addresses will always have four numeric components that
  // will have values between 0 and 255. If we wanted to store V4 addresses
  // as four u8 values but still express V6 addresses as one String value,
  // we wouldn’t be able to with a struct. Enums handle this case with ease:
  let home: IpAddr = IpAddr::V4(Ipv4Addr {
    address: (127, 0, 0, 1),
  });
  let loopback: IpAddr = IpAddr::V6(Ipv6Addr {
    address: String::from("::1"),
  });
  println!("Home IP Address is {:?}", home);
  println!("Loopback IP Address is {:?}", loopback);
  console::spacer();
}

#[derive(Debug)]
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

impl Message {
  fn call(&self) {
    println!("You called into a Message {:?}", self);
  }
}

// ### THE ABOVE enum Message ENCAPSULATES THE BELOW FOUR SEPARATE STRUCTS INTO A SINGLE TYPE...
// struct QuitMessage; // unit struct
// struct MoveMessage {
//   x: i32,
//   y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct

fn messages() {
  let quit_msg: Message = Message::Quit;
  let move_msg: Message = Message::Move { x: 1, y: 2 };
  let write_msg: Message = Message::Write(String::from("Hello world"));
  let change_color_msg: Message = Message::ChangeColor(1, 2, 3);
  println!("{:?}", quit_msg);
  println!("{:?}", move_msg);
  println!("{:?}", write_msg);
  println!("{:?}", change_color_msg);

  quit_msg.call();
  move_msg.call();
  write_msg.call();
  change_color_msg.call();
}

pub fn enums() {
  ip_addresses();
  messages();
}
