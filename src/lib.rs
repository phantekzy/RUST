// Traits as Parameters :
// Now that we know how to define and implement traits, we can explore how to use traits to define
// functions that accepts many different types.
// For example, in the past example, we implemented the Summary trait on the NewsArticle and Tweet
// types . We can define a notify function that calls the summarize method on its item parameter,
// which is of some type that implements the Summary trait.
//
//
//
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more {}...", self.summarize_author())
    }
}
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
    // Instead of a concrete type for the item parameter, we specify the impl keyword and the trait
    // name. This parameter accepts any type that implements the specified trait. In the body of
    // notify, we can call any methods on item that come from the Summary trait, such as summarize.
    // We can call notify and pass any instance of NewsArticle or Tweet. Code that calls the
    // function with any other type, such as a String or an i32, won't compile because those types
    // don't implement Summary.
}

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
