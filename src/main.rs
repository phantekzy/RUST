// Chapter 10 : Generic types ,traits , and lifetimes
// Traits : Defining Shared Behavior :
// Trait Bound Syntax :
// The impl Trait syntax works for straightforward cases but is actually syntax sugar for a longer
// form, which is called a trait bound; it looks like this :
//
//
//
//
// Now that we know how to define and implement traits, we can explore how to use traits to define
// functions that accept many different types.
// For Example, in a last example we implemented the Summary trait on the NewsArticle and Tweet
// types. We can define a notify function that calls the summarize method on its item parameter,
// which is of some type that implements the Summary trait.
//
//
//
//
//
//
//
use rusting::{NewsArticle, Summary, Tweet};

// Main function
fn main() {
    let _article = NewsArticle {
        headline: String::from("Maini Lotfi The cybersecurity master"),
        location: String::from("Algiers,Algeria"),
        author: String::from("John Doe"),
        content: String::from("Self made man "),
    };
    // This code prints New article available! (Read more...)
    // Creating a default implementation for summarize doesn't require to change anything about the
    // implementation of Summary on Tweet in the past example.
    //
    // After we define summarize_author, we can call summarize on instances of the Tweet struct, and
    // the default implementation of summarize will call the definition of summarize_author that
    // we've provided. Because we've implemented summarize_author, the Summary trait has given us
    // the Behavior of the summarize method without requiring us to write any more code.
    let tweet = Tweet {
        username: String::from("Rust_Ebook"),
        content: String::from("of course , as you probably already know , people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    // Note that it isn't possible to call the default implementation from an overriding
    // implementation of that same method.
}
