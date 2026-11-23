// Defining a Trait :
// Traits : Defining Shared Behavior :
// Trait Bound Syntax :
// Specifying Multiple Trait Bounds with the + Syntax :
//
//
pub fn notify<T: Summary>(item1: T, item2: T) {}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from{}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

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
