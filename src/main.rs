// Chapter 10 : Generic types ,traits , and lifetimes
// Traits : Defining Shared Behavior
// A trait tells the Rust compiler about functionality a particular type has and can share with
// other types. We can use traits to define shared behavior in an abstract way. We can use trait
// bounds to specify that Generic can be an type that has certain behavior
// NOTE : Traits are similar to a feature ofthen called interfaces in other languages, although with
// some differences .
// - Defining a Trait :

use rusting::{ID, Presnetation, Summary, Tweet};

// Main function
fn main() {
    // After implementing the trait, we can call the methods on instances of NewsArticle and Tweet in
    // the same way we call regular methods, like this :
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("Of course, as you probably already know,people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let id = ID {
        name: String::from("Maini Lotfi"),
        age: 30,
        _married: false,
    };
    println!("PRESENTATION : {}", id.pres());
}
