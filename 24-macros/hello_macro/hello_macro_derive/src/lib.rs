// Notice that we split the code into the hello_macro_derive function, which is responsible
// for parsing the TokenStream, and the impl_hello_macro function, which is responsible for
// transforming the syntax tree: this makes writing a procedural macro more convenient. The
// code in the outer function (hello_macro_derive in this case) will be the same for almost
// every procedural macro crate you see or create. The code you specify in the body of the
// inner function (impl_hello_macro in this case) will be different depending on your
// procedural macro’s purpose.

// The proc_macro crate comes with Rust, so we didn’t need to add that to the dependencies in
// Cargo.toml. The proc_macro crate is the compiler’s API that allows us to read and manipulate
// Rust code from our code.
use proc_macro::TokenStream;
use quote::quote;

// The hello_macro_derive function will be called when a user of our library specifies
// #[derive(HelloMacro)] on a type. This is possible because we’ve annotated the
// hello_macro_derive function here with proc_macro_derive and specified the name HelloMacro,
// which matches our trait name. This is the convention most procedural macros follow.
// ---
// Note that the output for our derive macro is also a TokenStream. The returned TokenStream
// is added to the code that our crate users write, so when they compile their crate, they’ll
// get the extra functionality that we provide in the modified TokenStream.
// ---
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
  // Construct a representation of Rust code as a syntax tree
  // that we can manipulate
  let ast = syn::parse(input).unwrap(); // results in a DeriveInput type, see below

  // Build the trait implementation
  impl_hello_macro(&ast)
}

// DeriveInput {
//     // --snip--

//     ident: Ident {
//         ident: "Pancakes",
//         span: #0 bytes(95..103)
//     },
//     data: Struct(
//         DataStruct {
//             struct_token: Struct,
//             fields: Unit,
//             semi_token: Some(
//                 Semi
//             )
//         }
//     )
// }

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;

  // The quote! macro lets us define the Rust code that we want to return.
  let gen = quote! {
    impl HelloMacro for #name {
      fn hello_macro() {
        println!("Hello, Macro! My name is {}", stringify!(#name));
      }
    }
  };
  gen.into() // ...and, we need to convert qoute! output to a TokenStream
}
