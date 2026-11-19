// Chapter 10 : Generic types ,traits , and lifetimes
// Traits : Defining Shared Behavior :
// - Default implementations :
//
//
use rusting::{NewsArticle, Summary, Tweet};

// Main function
fn main() {
    let article = NewsArticle {
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
}
