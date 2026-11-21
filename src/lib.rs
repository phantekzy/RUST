// Defining a Trait :
// Traits : Defining Shared Behavior :
// Trait Bound Syntax :
// The impl Trait syntax works for straightforward cases but is actually syntax sugar for a longer
// form, which is called a trait bound; it looks like this :
// pub fn notify (item: impl Summary){
// println!("Breaking news{}",item.summarize());
// }
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news!{}", item.summarize()) // This longer form is equivalent to the example
    // in the previous section but is more verbose .
    // We place trait bounds with the declaration of
    // the generic type parameter after a colon and
    // inside angle brakcets .
}

// Instead of a concrete type for the item parameter, we specify the impl keyword and the trait name
// This parameter accepts any type that implements the specified trait. In the body of notify we can
// call any methods on item that come from the Summary trait, such as summarize. We can call notify
// and pass in any instnace of NewsArticle or Tweet. Code that calls the function with any other
// type such as a String or an i32, won't compile because those types don't implement Summary.
//
//
// Sometimes it's useful to have default Behavior for some or all of the methods in a trait instead
// of requiring implementations for all methods on every type. Then, as we implement the trait on a
// particular type, we can keep or overrride each method's default behavior.
// This example shows how to specify a default string for the summarize method of the Summary trait
// instead of only defining the method signature
//
// The reason is that the syntax for overriding a default implementation is the same as the
// syntax for implementing a trait method that doesn't have a default implementation.
// Default implementation can call other methods in the same trait, even if those other methods
// don't have a default implementation. In this way, a trait can provide a lot of useful
// functionality and only require implementors to specify a small part of it. For example, we
// could define the Summary trait to have a summarize_author method whose implementation is
// required and then define a summarize method that has a default implementation that calls the
// summarize_author method :

pub trait Summary {
    fn summarize_author(&self) -> String;
    // Definition of a Summary trait with a default implementation of the summarize method
    fn summarize(&self) -> String {
        format!("(Read more from{}...)", self.summarize_author())
    } // To use this version of Summary, we only need to define summarize_author when we implement
    // the trait on a type :
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
