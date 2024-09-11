trait Pilot {
  fn fly(&self);
}

trait Wizard {
  fn fly(&self);
}

struct Human;

impl Human {
  fn fly(&self) {
    println!("*waving arms furiously*");
  }
}

impl Pilot for Human {
  fn fly(&self) {
    println!("This is your captain speaking.");
  }
}

impl Wizard for Human {
  fn fly(&self) {
    println!("Up!");
  }
}

// ## Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
pub fn same_name_methods() {
  println!("---------------------");
  let h = Human;

  // ## Khm, these are traits only, can't instantiate them
  // let p = Pilot;
  // let w = Wizard;
  // p.fly();
  // w.fly();

  // ## Specifying the trait name before the method name clarifies to Rust which implementation
  // ## of fly we want to call. We could also write Human::fly(&person), which is equivalent to
  // ## the person.fly(), but this is a bit longer to write if we don’t need to disambiguate.
  h.fly();
  // (h as Pilot).fly();
  // (h as Wizard).fly();
  Pilot::fly(&h);
  Wizard::fly(&h);
}

// ## However, associated functions that are not methods don’t have a self parameter. When there
// ## are multiple types or traits that define non-method functions with the same function name,
// ## Rust doesn’t always know which type you mean unless you use FULLY QUALIFIED SYNTAX.
// ---------------------------------------------------------------------------------------------
// ## In general, fully qualified syntax is defined as follows:
// <Type as Trait>::function(receiver_if_method, next_arg, ...);

trait Animal {
  fn baby_name() -> String;
}

struct Dog;

impl Dog {
  fn baby_name() -> String {
    String::from("Spot")
  }
}

impl Animal for Dog {
  fn baby_name() -> String {
    String::from("puppy")
  }
}

fn fully_qualified_syntax_disambiguation() {
  // ## Doesn't output what we want ('puppy'), but 'Spot' instead
  println!("A baby dog is called a {}", Dog::baby_name());

  // ## Doesn't compile - compiler doesn't know which implementation to use
  // println!("A baby dog is called a {}", Animal::baby_name());

  // ## To disambiguate and tell Rust that we want to use the implementation of Animal for Dog as
  // ## opposed to the implementation of Animal for some other type, we need to use fully qualified
  // ## syntax, indicating that we want to treat the Dog type as an Animal for this function call
  println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

pub fn run() {
  same_name_methods();
  fully_qualified_syntax_disambiguation();
}
