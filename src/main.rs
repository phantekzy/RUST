// Chapter 10 : Generic types ,trauts , and lifetimes
// Every programming language has tools for effectively handling the duplication of concepts .
// In Rust, one such tool is generics.
// Generics are abstract stand-ins for concrete types or other properties. When we're wrting code,
// we can express the behavior of generics or how they relate to other generics without lnowing what
// will be in their place when compiling and running the code .
//      Similar to the way a function takes parameters with unknown values to run the same code on
//      multiple concrete values, functions can take parameters of some generic typpes instead of a
//      concrete type, like i32 or String. In fact, we've already used before generics with
//      Option<T> and with Vec<T> and also with HashMap<K,V>, and finaly with Result<T,E>.
//      We will explore how to define our own types,functionss and methods with generics!
//
//  First, we'll review how to extract a function to reducve code duplication.
//  Next, we'll use the same technique to make a generic function from two functions that differ
//  onlu in the types of their parameters. We'll also explain how to use generic types in struct and
//  enum definitions.
//  Then we'll learn how to use traits to define behavior in a generic way.we can combine traits
//  with with generic types to constrain a generic type to only those types that have a particular
//  behavior , as opposed to just any type.
//  Finally, we'll discuss lifetimes , a variety of generics that give the compiler information
//  about how references relate to each other.
// Main function
fn main() {}
