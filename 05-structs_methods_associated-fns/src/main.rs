// use derive debug so at least some automated output of the struct can be made
#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn debugging_structs() {
  // Another way to print out a value using the Debug format is to use the dbg! macro,
  // which takes ownership of an expression (as opposed to println!, which takes a reference),
  // prints the file and line number of where that dbg! macro call occurs in your code along
  // with the resultant value of that expression, and returns ownership of the value.
  // ---------------------------------------------------------------------------------------
  // Note: Calling the dbg! macro prints to the standard error console stream (stderr),
  // as opposed to println!, which prints to the standard output console stream (stdout)

  let scale = 2;
  let rect = Rectangle {
    width: dbg!(40 * scale),
    height: 12,
  };

  dbg!(&rect); // borrow, so that it doesn't destroy 'rect'
}

fn area_from_simple_vars(width: u32, height: u32) -> u32 {
  width * height
}

fn area_from_tuple(dimensions: (u32, u32)) -> u32 {
  // as tuples don’t name their elements, we have to index into
  // the parts of the tuple, making our calculation less obvious.
  dimensions.0 * dimensions.1

  // Mixing up the width and height wouldn’t matter for the area calculation, but
  // if we want to draw the rectangle on the screen, it would matter! We would
  // have to keep in mind that width is the tuple index 0 and height is the tuple
  // index 1. This would be even harder for someone else to figure out and keep
  // in mind if they were to use our code. Because we haven’t conveyed the meaning
  // of our data in our code, it’s now easier to introduce errors.

  // ## variation with extracting the meaning
  // let (width, height) = dimensions;
  // width * height
}

// we don't want/need ownership, so let's take it as a reference to only [immutable] borrow.
// That way, main would be able to continue using the structure, for instance in the println!()
fn area_from_struct(rect: &Rectangle) -> u32 {
  rect.width * rect.height
}

// ------------------------------------------------------------------------------------
// METHOD SYNTAX: Unlike functions, methods are defined within the context of a struct
// (or an enum or a trait object, covered later), and their first parameter is always
// self, which represents the instance of the struct the method is being called on.
// ------------------------------------------------------------------------------------
// ASSOCIATED FUNCTIONS: All functions defined within an impl block are called
// associated functions because they’re associated with the type named after the impl.
// We can define associated functions as functions that don’t have self as their first
// parameter (and thus are not methods) because they don’t need an instance of the type
// to work with. We’ve already used one function like this: the String::from function
// that’s defined on the String type.
// ------------------------------------------------------------------------------------
impl Rectangle {
  // To define the function within the context of Rectangle, we start an impl
  // (implementation) block for Rectangle. Everything within this impl block
  // will be associated with the Rectangle type.

  // The &self is actually short for self: &Self. Within an impl block,
  // the type Self is an alias for the type that the impl block is for
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn set_width(&mut self, w: u32) {
    self.width = w;
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }

  // Note that we still need to use the & in front of the self shorthand to indicate
  // that this method borrows the Self instance, just as we did in rectangle: &Rectangle.
  // Methods can take ownership of self, borrow self immutably, as we’ve done here,
  // or borrow self mutably, just as they can any other parameter.

  // Having a method that takes ownership of the instance by using just self as
  // the first parameter is rare; this technique is usually used when the method
  // transforms self into something else and you want to prevent the caller
  // from using the original instance after the transformation.

  // ------------------------------------------------------------------------------------

  // Associated functions that aren’t methods are often used for constructors that will
  // return a new instance of the struct. These are often called new, but new isn’t a
  // special name and isn’t built into the language.
  // For example, we could choose to provide an associated function named square that would
  // have one dimension parameter and use that as both width and height, thus making it
  // easier to create a square  Rectangle rather than having to specify the same value twice:
  fn square(size: u32) -> Self {
    // Rectangle {
    Self {
      width: size,
      height: size,
    }
  }
}

fn methods() {
  let r1 = Rectangle {
    width: 30,
    height: 50,
  };
  let r2 = Rectangle {
    width: 10,
    height: 40,
  };
  let r3 = Rectangle {
    width: 60,
    height: 45,
  };

  println!("Can r1 hold r2? {}", r1.can_hold(&r2));
  println!("Can r1 hold r3? {}", r1.can_hold(&r3));

  // Method calls are SYNTACTIC SUGAR for Function calls:
  println!("Can r1 hold r2? {}", Rectangle::can_hold(&r1, &r2));
  println!("Can r1 hold r3? {}", Rectangle::can_hold(&r1, &r3));
}

// A function defined on the given Type which is NOT a method (doesn't operate)
// on an instance. Calling syntax for such functions is <Type>::<function>(...args)
fn associated_functions() {
  let sq = Rectangle::square(24);
  println!(
    "Area of the square {:?} is: {} square pixels",
    sq,
    sq.area()
  );

  // Method calls are SYNTACTIC SUGAR for Function calls:
  let mut r = Rectangle {
    width: 7,
    height: 5,
  };
  let area1 = r.area();
  let area2 = Rectangle::area(&r);
  assert_eq!(area1, area2);
  println!("{} {}", area1, area2);

  r.set_width(2);
  let area1 = r.area();
  let area2 = Rectangle::area(&r);
  assert_eq!(area1, area2);
  println!("{} {}", area1, area2);

  // DEREFERENCING A POINTER ACCESS
  // As we described in Chapter 4.3 "Dereferencing a Pointer Accesses Its Data", Rust will
  // insert as many references and dereferences as needed to make the types match up for
  // the self parameter.
  // For example, here are two equivalent calls to area for a mutable reference to a boxed rectangle:
  let r = &mut Box::new(Rectangle {
    width: 1,
    height: 2,
  });
  let area1 = r.area();
  let area2 = Rectangle::area(&**r);
  assert_eq!(area1, area2);
  println!("{} {}", area1, area2);
}

fn main() {
  let width_px: u32 = 33;
  let height_px: u32 = 12;
  let area_vars = area_from_simple_vars(width_px, height_px);
  println!("Area from simple vars: {} square pixels", area_vars);

  let rect_tuple = (33, 12);
  let area_tuple = area_from_tuple(rect_tuple);
  println!("Area from tuple: {} square pixels", area_tuple);

  let rect = Rectangle {
    width: 33,
    height: 12,
  };
  let area_struct = area_from_struct(&rect);
  println!("Area from struct: {} square pixels", area_struct);
  println!("Area via struct method: {} square pixels", rect.area());

  // rect printout using derive(debug)
  println!("Area from struct {:?}: {} square pixels", rect, area_struct);
  // alternative printout formatting
  println!(
    "Area from struct {:#?}: {} square pixels",
    rect, area_struct
  );

  // Putting the specifier :? inside the curly brackets tells println! we want
  // to use an output format called Debug. The Debug trait enables us to print
  // our struct in a way that is useful for developers so we can see its value
  // while we’re debugging our code.

  println!("--------------------");
  debugging_structs();

  println!("--------------------");
  methods();
  associated_functions();
}
