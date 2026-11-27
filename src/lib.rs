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
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
} // Generic Type -> Trait Bound 
// This longer form is equivalent to the example in the previous section but is more verbose. We
// place trait bounds with the declaration of the generic type parameter after a colon and inside
// angle brackets.

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
