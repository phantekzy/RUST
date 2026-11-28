use std::fmt::Display;

// Returning Types that implement Traits
// We can also use the impl Trait syntax in the return position to return a value of some type that
// implements a trait, as shown here :
fn _returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("Phantekzy"),
        content: String::from("How to code in Rust"),
        reply: false,
        retweet: true,
    }
}
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more {}...", self.summarize_author())
    }
}
pub fn notify<T: Summary + Display>(_item: T) {}
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
