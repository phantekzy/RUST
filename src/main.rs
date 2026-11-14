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
// Let's look how this works with an example that uses the standard library's Option<T> enum :

// Main function
fn main() {
    let integer = Some(5);
    let float = Some(5.0);
    // When Rust compiles this code, it performs monomorphization. During this process, the compiler
    // reads the values that have been used in Option<T> instances and identifies two kinds of
    // Option<T>: one is i32 and the other is f64.
    // As such, it expands the generic definition of Option<T> into Option_i32 and Option_f64,
    // thereby replacing the generic definition with the specific ones.
}
// The monomorphized version of ther code looks like the following. The generic Option<T> is
// replaced with the specific definition created by the compiler :
enum Option_i32 {
    Some(i32),
    None,
}
enum Option_f64 {
    Some(f64),
    None,
}
