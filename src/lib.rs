// Default implementation :
// Sometimes it's usefull to have default behavior for some or all of the methods in a trait instead
// of requiring implementations for all methods on every type. Then, as we implement the trait on a
// particular type, we can keep or override each method's default behavior.
// This example shows how to specify a default string for the summarize method of the Summary trait
// instead of only defining the method signature
//
// Default implementations can call other methods in the same trait, even if those other methods
// don't have a default implementation. In this way, a trait can provide a lot of useful
// functionality and only require implementors to specify a small part of it. For example, we
// could define the Summary trait to have a summarize_author method whose implementation is
// required, and then define a summarize method that has a default implementation that calls the
// summarize_author method :
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Read more...")
    } // Definition of a Summary trait with a default implementation of the summarize method
}
// To use a default implementation to summarize instances of NewsArticle instead of defining a
// custom implementation , we specify an empty impl block with impl Summary for NewsArticle{}
impl Summary for NewsArticle {}
//
//
// NewsArticle Struct :
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Tweet Struct
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
} // Implementing the Summary trait on the NewsArticle type
// Implementing a trait on a type is similar to implementing regular methods. The difference is that
// after impl, we put the trait name that we want to implement, then use the the for keyword, and
// them specify the name of the type we want to implement the trait for. Within the impl block, we
// put the method signatures that the trait definition has defined. Instead of adding a semucolon
// after each signature, we use curly brackets  and fill in the method body with the specific
// behavior that we want the methoods of the trait to have for the particular type.
//
