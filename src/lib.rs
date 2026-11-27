// Traits as Parameters :
// Trait Bound Syntax :
// The impl trait Syntax works for straightforward cases but is actually syntax sugar for a longer
// form, which is called a trait bound; it looks like this :
//
//
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more {}...", self.summarize_author())
    }
}
// This longer form is equivalent to the example in the previous section but is more verbose. We
// place trait bounds with the declaration of the generic type parameter after a colon and inside
// angle brackets.
// The impl Trait syntax is convenient and makes for more concise code in simple cases. The trait
// bound syntax can express more complexity in other cases. For example, we can have two Parameters
// that implement Summary.
// using the impl Trait syntax looks like this :
// pub fn notify(item1: impl Summary, item2: impl Summary) {}
// If we wanted this function to allow item1 and item2 to have different types, using impl Trait
// would be appropriate (as long as both types implement Summary). If we wanted to force both
// parameters to have the same type, that's only possible to express using trait bound like this :
pub fn notify<T: Summary>(item1: T, item2: T) {}

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
