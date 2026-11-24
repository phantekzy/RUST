// Chapter 10 : Generic types ,traits , and lifetimes
// Traits : Defining Shared Behavior :
// Specifying Multiple Trait Bounds with the + Syntax :

// The Difference between "Normal trait parameters and Trait bounds" :
// In Rust the Difference between them lies in concrete vs Generic resolution and dispatch strategy
//
// 1. impl trait in parameters(Anonymous Generic) :
// When you use impl Trait in a function argument, it is syntactic sugar for a Generic type, but
// each argument can be a different concrete type. The compiler generates a separate copy of the
// function for each unique type used (monomorphization)
trait Summary {
    fn summarize(&self) -> String;
}
// Main function
fn main() {}
