// Chapter 10 : Generic types ,trauts , and lifetimes
// Generic Data Types
// Performance of code Using Generics :
// We might be wondering whether there is a runtime cost when we're using generic type parameters.
// The good news is that Rust implements generics in such a way that our code doesn't run any slower
// using generic types than it would with concrete types.
// Rust acomplishes this by performing monomorphization of the code that is using generics at
// compile time . monomorphization is the process of turning generic code into specific code by
// filling in the concrete thypes that are used when compiled.

// Main function
fn main() {}
