
Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs.

These features, sometimes collectively referred to as the module system, include:

  - PACKAGES: A Cargo feature that lets you build, test, and share crates
  - CRATES: A tree of modules that produces a library or executable
  - MODULES and USE: Let you control the organization, scope, and privacy of paths
  - PATHS: A way of naming an item, such as a struct, function, or module


A CRATE can come in one of two forms: a BINARY CRATE or a LIBRARY CRATE.

  Binary crates are programs you can compile to an executable that you can run, such as a command-line program or a server. Each MUST HAVE A FUNCTION CALLED main() that defines what happens when the executable runs.

  Library crates DON’T HAVE A MAIN FUNCTION, and they don’t compile to an executable. Instead, they define functionality intended to be shared with multiple projects.

For example, the rand crate we used in Chapter 2 provides functionality that generates random numbers. Most of the time when Rustaceans say “crate”, they mean library crate, and they use “crate” interchangeably with the general programming concept of a “library". Also, all the crates we’ve created so far have been binary crates.


A PACKAGE
  - is a bundle of one or more crates that provides a set of functionality.
  - contains a Cargo.toml file that describes how to build those crates.
  - Cargo is actually a package that contains the binary crate for the command-line
    tool you’ve been using to build your code.
  - The Cargo package also contains a library crate that the binary crate depends on.
  - Other projects can depend on the Cargo library crate to use the same logic the
    Cargo command-line tool uses.

A PACKAGE
  - can contain AS MANY BINARY CRATES as you like, but AT MOST ONLY ONE LIBRARY CRATE.
  - must contain at least one crate, whether that’s a library or binary crate.


CARGO follows a CONVENTION that
  - src/main.rs is the crate root of a binary crate with the same name as the package.
  - src/lib.rs (if contained in the package) is the crate root of a library crate with
    the same name as the package
  - Cargo passes the crate root files to rustc to build the library or binary.


A PATH can take two forms:

  - An absolute path is the full path starting from a crate root
      - for code from an external crate, the absolute path begins with the crate name, and
      - for code from the current crate, it starts with the literal crate.
  - A relative path starts from the current module
      - uses self, super, or an identifier in the current module.
  - Our preference in general is to specify absolute paths because it’s more likely we’ll
    want to move code definitions and item calls independently of each other.

IDIOMATIC USE of Paths discussed in:
https://rust-book.cs.brown.edu/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#creating-idiomatic-use-paths


MODULES' PRIVACY:
  - Items in a parent module can’t use the private items inside child modules
  - but items in child modules can use the items in their ancestor modules.
  - This is because child modules wrap and hide their implementation details, but the
    child modules can see the context in which they’re defined.

Making Structs and Enums Public:
  - If we use pub before a struct definition, we make the struct public
      - but the struct’s fields will still be private
      - we can make each field public or not on a case-by-case basis.
  - In contrast, if we make an enum public, all of its variants are then public
      - we only need the pub before the enum keyword

Enums aren’t very useful unless their variants are public; it would be annoying to have to annotate all enum variants with pub in every case, so the default for enum variants is to be public. Structs are often useful without their fields being public, so struct fields follow the general rule of everything being private by default unless annotated with pub.

SUMMARY:
  - If we have a package that only contains src/main.rs, it means it only contains
    a binary crate named my-project.
  - If a package contains src/main.rs and src/lib.rs, it has two crates: a binary and a library,
    both with the same name as the package.
  - A package can have multiple binary crates by placing files in the src/bin directory;
    each file will be a separate binary crate.

Modules organization & Cheatsheet:
https://rust-book.cs.brown.edu/ch07-02-defining-modules-to-control-scope-and-privacy.html


BEST PRACTICES for Packages with a Binary and a Library

    We mentioned a package can contain both a src/main.rs binary crate root as well as a src/lib.rs library crate root, and both crates will have the package name by default. Typically, packages with this pattern of containing both a library and a binary crate WILL HAVE JUST ENOUGH CODE in the binary crate to START AN EXECUTABLE THAT CALLS CODE WITH THE LIBRARY CRATE.

    This lets other projects benefit from the most functionality that the package provides, because the library crate’s code can be shared. The module tree should be defined in src/lib.rs. Then, any public items can be used in the binary crate by starting paths with the name of the package.

    The binary crate becomes a user of the library crate just like a completely external crate would use the library crate: it CAN ONLY USE THE PUBLIC API. This helps you design a good API; not only are you the author, you’re also a client!


Using EXTERNAL PACKAGES

To use an external package called rand to get random numbers in our project,
we need to add rand as a dependency to Cargo.toml by adding this line:

    rand = "0.8.5"

That tells Cargo to download the rand package and any dependencies from
crates.io and make rand available to our project.

Then, to bring rand definitions into the scope of our package, we added a 'use'
line starting with the NAME OF THE CRATE, rand, and LISTED THE ITEMS we wanted
to bring into scope. Recall that in the “Generating a Random Number” section in
Chapter 2, we brought the Rng trait into scope and called the rand::thread_rng function:

    use rand::Rng;

    fn main() {
        let secret_number = rand::thread_rng().gen_range(1..=100);
    }


Note that the standard std library is also a crate that’s external to our package.
Because the standard library is shipped with the Rust language, we don’t need to
change Cargo.toml to include std. But we do need to refer to it with use to bring
items from there into our package’s scope. For example, with HashMap we would use this line:

    use std::collections::HashMap;

This is an absolute path starting with std, the name of the standard library crate.

If we’re using multiple items defined in the same crate or same module, listing each
item on its own line can take up a lot of vertical space in our files. Instead, we
can use nested paths to bring the same items into scope in one line:

    use std::cmp::Ordering;
    use std::io;
    ---- becomes: ----
    use std::{cmp::Ordering, io};

We can also use 'self' in the nested path:

    use std::io;
    use std::io::Write;
    ---- becomes: ----
    use std::io::{self, Write};


THE GLOB OPERATOR:

If we want to bring all public items defined in a path into scope, we can specify that path
followed by the * glob operator. Be careful when using the glob operator! Glob can make it
harder to tell what names are in scope and where a name used in your program was defined.

    use std::collections::*;

The glob operator is often used when testing to bring everything under test into the tests
module. The glob operator is also sometimes used as part of the prelude pattern: see the
standard library documentation for more information on that pattern.

