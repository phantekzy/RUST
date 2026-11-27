// Chapter 10 : Generic types ,traits , and lifetimes
// Traits : Defining Shared Behavior
// A trait tells the Rust compiler about functionality a particular type has and can share with
// other types. We can use traits to define shared behavior in an abstract way. We can use trait
// bounds to specify that Generic can be an type that has certain behavior
// NOTE : Traits are similar to a feature ofthen called interfaces in other languages, although with
// some differences .
// - Defining a Trait :

use rusting::{Summary, Tweet};

// Main function
fn main() {
    // After we define summarize_author, we can call summarize on instances of the Tweet struct, and
    // the default implementation of summarize will call the definition of summarize_author that
    // we've provided . because we've implemented summarize_author, the Summary trait has given us
    // the behavior of the summarize method without requiring us to write any more code.
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("Of course, as you probably already know,people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}
