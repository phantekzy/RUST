// Chapter 10 : Generic types ,traits , and lifetimes
// Traits : Defining Shared Behavior :
// - Default implementations :
//
//
use rusting::NewsArticle;
use rusting::Summary;

// Main function
fn main() {
    let article = NewsArticle {
        headline: String::from("Maini Lotfi The cybersecurity master"),
        location: String::from("Algiers,Algeria"),
        author: String::from("John Doe"),
        content: String::from("Self made man "),
    };
    println!("New article available ! {}", article.summarize())
    // This code prints New article available! (Read more...)
    // Creating a default implementation for summarize doesn't require to change anything about the
    // implementation of Summary on Tweet in the past example.
    // The reason is that the syntax for overriding a default implementation is the same as the
    // syntax for implementing a trait method that doesn't have a default implementation.
}
