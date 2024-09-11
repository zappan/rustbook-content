// One increasingly popular approach to ensuring safe concurrency is message passing,
// where threads or actors communicate by sending each other messages containing data.
// Here’s the idea in a slogan from the Go language documentation: “Do not communicate
// by sharing memory; instead, share memory by communicating.” To accomplish message-sending
// concurrency, Rust’s standard library provides an implementation of channels. A channel
// is a general programming concept by which data is sent from one thread to another.
// --------------------------------------------------------------------------------------
// !!! CHANNELS CAN ONLY SEND VALUES OF A SINGLE TYPE !!!
// ... no different types per different message to send around ...
// If you want to send values of multiple types, you can use either an enum or the Any trait.
// --------------------------------------------------------------------------------------

// multi-threaded channel module; mpsc = multiple-producer-single-consumer
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const THREAD_WAIT_MILLIS: u64 = 60;

fn channel_communication() {
  let ch = mpsc::channel();
  let (tx, rx) = ch;

  thread::spawn(move || {
    let val = String::from("Hi");

    // ## what about ownership? Borrowed-before-move -> OK
    println!("Thread is sending: {val}");

    // In this example, we’re calling unwrap to panic in case of an error.
    // But in a real application, we would handle it properly
    tx.send(val).unwrap(); // will infer the type in the channel definition from this send...
                           // also, note that send() takes ownership, MOVES the data
                           // and, in the end, receiver takes ownership of it

    // ## what about ownership? Borrowed-after-move error
    // println!("Sent: {val}");
  });

  // as with tx, we're calling unwrap as an example, rather than handling it properly
  let received_msg = rx.recv().unwrap();
  println!("Message received from the thread: {received_msg}");
}

fn concurrent_communication() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    // let vals: Vec<_> = "Hi from the thread".split(" ").map(String::from).collect();
    let vals: Vec<_> = "Hi from the thread".split(" ").collect();
    thread::sleep(Duration::from_millis(THREAD_WAIT_MILLIS));

    for val in vals {
      println!("Thread sending: {val}");
      tx.send(val).unwrap();
      thread::sleep(Duration::from_millis(THREAD_WAIT_MILLIS));
    }
  });

  // Here, we’re treating rx as an iterator. For each value received,
  // we’re printing it. When the channel is closed, iteration will end.
  for received in rx {
    println!("Got: {received}");
  }
}

fn multiple_producers() {
  let (tx1, rx) = mpsc::channel();
  let tx2 = tx1.clone(); // we call clone on the transmitter. This will
                         // give us a new transmitter we can pass to the first spawned thread

  thread::spawn(move || {
    let vals: Vec<_> = "Hi from the thread".split_whitespace().collect();
    for val in vals {
      tx1.send(val).unwrap();
      thread::sleep(Duration::from_millis(THREAD_WAIT_MILLIS));
    }
  });

  thread::spawn(move || {
    let vals: Vec<_> = "we've got more messages for you"
      .split_whitespace()
      .collect();
    for val in vals {
      tx2.send(val).unwrap();
      thread::sleep(Duration::from_millis(THREAD_WAIT_MILLIS));
    }
  });

  for received in rx {
    println!("Got: {received}");
  }
}

pub fn run() {
  // !!! CHANNELS CAN ONLY SEND VALUES OF A SINGLE TYPE !!!
  channel_communication();
  println!("---------------------");
  concurrent_communication();
  println!("---------------------");
  multiple_producers();
}
