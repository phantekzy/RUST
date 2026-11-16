// Chapter 10 : Generic types ,traits , and lifetimes
// Traits : Defining Shared Behavior :
// - Default implementations :
// Sometimes it's useful to have default behavior for some or all of the methods in a trait instead
// of requiring implementations for all methods on every type.
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
// A trait tells the Rust compiler about functionality a particular type has and can share with
// other types. We can use traits to define shared behavir in an abstract way. We can use trait
// bounds to specify that a generic can be any type that has certain behavior.
// // NOTE : Traits are similar to a feature often called interfaces in other languages, although
// with some differences.
//
//
use rusting::{Summary, Tweet};
// Main function
fn main() {
    // After implementing the trait, we can call the methods on instances of NewsArticle and Tweet in
    // the same way we call regular methods, like this :
    let tweet = Tweet {
        username: String::from("Rust_book"),
        content: String::from("Learn how to be a Rustacean !"),
        reply: true,
        retweet: true,
    };
    println!("1 new tweet :{}", tweet.summarize())
    // This code prints 1 new tweet: Rust_book: Learn how to be a Rustacean.
    // Note that because we defined the Summary trait and the NewsArticle and Tweet types in the
    // same lib.rs, they're all in the same scope. let's say this lib.rs is for a crate we've called
    // aggregator and someone  else wants to use our crate's functionality to implement the Summary
    // trait on a struct defined within their library's scope. They would need to bring the trait
    // into their scope first. They would do so by specifying use rusting::Summary;, which then
    // would enable them to implement Summary for their type. The Summary trait would also need to
    // be a public trait for another crate to implement it, which it is because we put the pub
    // keyword before trait.
    // One restriction to note with trait implementations is that we can implement a trait on a type
    // only if either the trait or the type is local to our crate. For example, we can implement
    // standard library traits like Display on a custom type like Tweet as part of our rusting crate
    // functionality, because the type Tweet is local to our rusting crate. We can also implement
    // Summary on Vec<T> in our rusting crate because the trait Summary is local to our crate.
    // But we can't implement external traits on external types. for example, we can't implement the
    // Display trait on Vec<T> within our rusting crate, because Display and Vec<T> are defined in
    // the standard library and aren't local to our rusting crate. This restriction is part of a
    // property of programs called 'coherence', and more specifically the orphan rule, so named
    // because the parent type is not present. This rule ensures that other people's code can't
    // break our code and vice versa. Without the rule, two crates could implement the same trait
    // for the same type, and Rust wouldn't know which implementation to use.
}
