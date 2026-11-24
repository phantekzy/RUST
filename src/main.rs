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
// Each argument can be a DIFFERENT type that implements Summary
//fn notify(item1: &impl Summary, item2: &impl Summary) {
//   println!("{}", item1.summarize());
//  println!("{}", item2.summarize());
//}
// 2. Trait Bounds with Generics(T: Trait) :
// When you use a Generic type parameter T  with a trait bound, all occurrences of T in that scope
// must be the same concrete type. This is stricter than impl Trait in arguments.
fn notify<T: Summary>(item1: &T, item2: &T) {
    println!("{}", item1.summarize());
    println!("{}", item2.summarize());
}
// Main function
fn main() {}
