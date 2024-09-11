#[derive(Debug, Clone, Copy)]
enum ShirtColor {
  Red,
  Blue,
}

struct Inventory {
  shirts: Vec<ShirtColor>,
}

impl Inventory {
  fn giveaway(&self, user_pref: Option<ShirtColor>) -> ShirtColor {
    // this is a closure under else
    user_pref.unwrap_or_else(|| self.most_stocked())
  }

  fn most_stocked(&self) -> ShirtColor {
    let mut red_count: u32 = 0;
    let mut blue_count: u32 = 0;
    for color in &self.shirts {
      match color {
        ShirtColor::Red => red_count += 1,
        ShirtColor::Blue => blue_count += 1,
      }
    }
    if red_count > blue_count {
      ShirtColor::Red
    } else {
      ShirtColor::Blue
    }
  }
}

fn store_giveaway() {
  let store: Inventory = Inventory {
    shirts: vec![
      ShirtColor::Blue,
      ShirtColor::Red,
      ShirtColor::Red,
      ShirtColor::Red,
      ShirtColor::Blue,
      ShirtColor::Red,
    ],
  };

  let user_pref1: Option<ShirtColor> = Some(ShirtColor::Blue);
  let giveaway1 = store.giveaway(user_pref1);
  println!(
    "The user with preference {:?} gets {:?}",
    user_pref1, giveaway1
  );

  let user_pref2: Option<ShirtColor> = None;
  let giveaway2 = store.giveaway(user_pref2);
  println!(
    "The user with preference {:?} gets {:?}",
    user_pref2, giveaway2
  );
}

fn explicit_closure() {
  // explicit closure annotations make it look more like a function
  let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    std::thread::sleep(std::time::Duration::from_millis(700));
    num
  };

  println!("{}", expensive_closure(32));

  // # These are all valid definitions that will produce the same behavior when they’re called.
  // # The add_one_v3 and add_one_v4 lines require the closures to be evaluated to be able to
  // # compile because the types will be inferred from their usage
  // fn  add_one_v1   (x: u32) -> u32 { x + 1 }
  // let add_one_v2 = |x: u32| -> u32 { x + 1 };
  // let add_one_v3 = |x|             { x + 1 };
  // let add_one_v4 = |x|               x + 1  ;

  // # For closure definitions, the compiler will infer one concrete type
  // # for each of their parameters and for their return value.
  let example_closure = |x| x;
  // # Because there are no type annotations, we can call the closure with any type
  let s = example_closure(String::from("hello"));
  // # The type(s) gets locked into the closure with the first call,
  // # thus the following call will cause an error, as type has been inferred by the above call
  // let n = example_closure(5);
  println!("{}", s);
}

fn closures_parameters_capturing() {
  println!("---------------------------------------------");

  // The closure will decide which of the ways to take a parameter (borrowing immutably,
  // borrowing mutably, and taking ownership) to use based on what the body of the function
  // does with the captured values.
  let list = vec![1, 2, 3];
  println!("Before defining closure: {:?}", list);
  let only_borrows = || println!("From closure: {:?}", list);
  println!("Before calling closure: {:?}", list);
  only_borrows();
  println!("After calling closure: {:?}", list);

  println!("---------------------------------------------");

  let mut list = vec![1, 2, 3];
  println!("Before defining closure: {:?}", list);
  let mut borrows_mutably = || list.push(9);
  // ## cannot call the following because it would borrow immutably,
  // ## preventingmutable borrow required by the closure
  // println!("Before calling closure: {:?}", list);
  borrows_mutably();
  // ## mutable borrows ends here, as we don't use the closure again after it's closed
  // ## thus the following println!() can freely borrow immutably
  println!("After calling closure: {:?}", list);

  println!("---------------------------------------------");

  // ## If you want to force the closure to take ownership of the values it uses in the
  // ## environment even though the body of the closure doesn’t strictly need ownership,
  // ## you can use the move keyword before the parameter list.
  let list = vec![1, 2, 3];
  println!("Before defining closure: {:?}", list);
  // ## the list moves (main thread passes ownership to the spawn thread)
  std::thread::spawn(move || {
    std::thread::sleep(std::time::Duration::from_millis(240));
    println!("From a threaded closure: {:?}", list);
  })
  .join()
  .unwrap();
  // ## The following println!() errors because the variable moved (changed ownership)
  // println!("After calling closure: {:?}", list);
  println!("Main thread finishes work... in this case, explicitly waited for the spawned one (join()) to finish");
}

pub fn run() {
  store_giveaway();
  explicit_closure();
  closures_parameters_capturing();
}
