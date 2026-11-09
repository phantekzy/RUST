// Chapter 10 : Generic types ,trauts , and lifetimes
// Generic Data Types
// In Enum Definitions
// As we did with structs, we can define enms to hold generic data types in their variants, let's
// take another look at tthe Option<T> enum that the standard library provides :
enum Option<T> {
    Some(T),
    None,
} // The definition should now make more sese to us. A we can see Option<T> is an enum that is
// generic over type T and has two variants: Some, which holds one value of type T, and a None
// variant that doesn't hold any value.
// Main function
fn main() {}
