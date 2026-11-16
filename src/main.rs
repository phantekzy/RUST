// Chapter 10 : Generic types ,traits , and lifetimes
// Traits : Defining Shared Behavior :
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
    // into their scope first.
    //
}
