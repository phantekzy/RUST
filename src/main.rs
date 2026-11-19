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
}
