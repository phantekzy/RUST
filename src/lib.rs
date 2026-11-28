use std::fmt::Display;

// Clearer Trait Bounds with where Clauses
// Using too many Trait bounds has its downsides. Each generic has its own trait bounds, so
// functions with multiple generic type parameters can contain lots of trait bound information
// between the function's name and its parameter list, making the function signature hard to read.
// For this reason, Rust has alternate syntax for specifying trait bounds inside a where clause
// after the function signature
//
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more {}...", self.summarize_author())
    }
}
pub fn notify<T: Summary + Display>(item: T) {}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
// NewsArticle Struct :
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
