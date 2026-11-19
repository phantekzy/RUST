// Defining a Trait :
// Traits : Defining Shared Behavior :
// - Default implementations :
// Sometimes it's useful to have default Behavior for some or all of the methods in a trait instead
// of requiring implementations for all methods on every type. Then, as we implement the trait on a
// particular type, we can keep or overrride each method's default behavior.
// This example shows how to specify a default string for the summarize method of the Summary trait
// instead of only defining the method signature

pub trait Summary {
    // Definition of a Summary trait with a default implementation of the summarize method
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
// To use a default implementation to summarize instances of NewsArticle instead of defining a
// custom implementation, we specify an empty impl block with impl Summary for NewsArticle{}.
// Even though we're no longer defining the summarize
//  method on NewsArticle directly, we've provided a default implementation and specified that
//  NewsArticle implements the Summary trait . As a result, we can still call the summarize method
//  on an instance of NewsArticle,

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {}
