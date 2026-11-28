use std::fmt::Display;

// Specifying Multiple Trait Bounds with the + Syntax
// We can also specify more than one trait bound. Say we wanted notify to use display formatting on
// item as well as the summarize method : we specify in the notify definition that item must
// implement both Display and Summary. We can do so using the + Syntax:
//
//
//
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more {}...", self.summarize_author())
    }
}
// pub fn notify(item: impl Summary + Display) { }
// The + Syntax is also valid with trait bounds on generic types :
pub fn notify<T: Summary + Display>(item: T) {} // With the two trait bounds specified, the body of
// notify can call summarize and use {} to format item.
// Tweet Struct
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
