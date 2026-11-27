use rusting::{Summary, Tweet};
// Chapter 10 : Generic types ,traits , and lifetimes
// Traits : Defining Shared Behavior
// Specifying Multiple Trait Bounds with the + Syntax
// Clearer Trait Bounds with where Clauses :
// Returning Types that implement Traits
// We can also use the impl Trait Syntax in the return position to return a value of some type that
// implements a trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("Rust Books"),
        content: String::from("Programming language official book"),
        reply: false,
        retweet: true,
    }
}
// By using impl Summary for the return type, we specify that the returns_summarizable function
// returns some type that implements the Summary trait without naming the concrete type.
// Main function
fn main() {}
