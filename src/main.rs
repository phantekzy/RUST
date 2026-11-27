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
// returns some type that implements the Summary trait without naming the concrete type. In this
// case, returns_summarizable returns a Tweet, but the code calling this function doesn't know that.
// The ability to return a type that is only specified by the trait it implements is especially
// usefull in the context of closures and iterators, which we will cover soon. Closures and
// iterators create types that only the compilers knows or types that are very long to specify.
//
//
//
// Main function
fn main() {}
