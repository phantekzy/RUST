// Chapter 10 : Generic types ,trauts , and lifetimes
// Generic Data Types
// Performance of code Using Generics :
// We might be wondering whether there is a runtime cost when we're using generic type parameters.
// The good news is that Rust implements generics in such a way that our code doesn't run any slower
// using generic types than it would with concrete types.
// Rust acomplishes this by performing monomorphization of the code that is using generics at
// compile time . monomorphization is the process of turning generic code into specific code by
// filling in the concrete thypes that are used when compiled.
// In this process, the compiler does the opposit of the steps we used to create generic functions :
// the compiler looks at all the places where generic code is called and generates code for the
// concrete types the generic code is called with.

// Main function
fn main() {}
