// Chapter 10 : Generic types ,traits , and lifetimes
// Traits : Defining Shared Behavior :
// A trait tells the Rust compiler about functionality a particular type has and can share with
// other types. We can use traits to define shared behavir in an abstract way. We can use trait
// bounds to specify that a generic can be any type that has certain behavior.
// // NOTE : Traits are similar to a feature often called interfaces in other languages, although
// with some differences.
//
// Defining a Trait :
// A type's behavior consists of the methods we can call on that type. Different types share the
// same behavior if we can call the same methods on all of those types. TGrait definitions are a way
// to group method signatures together to define a set of behaviors necessary to accompl;ish some
// purpose.
// For example, let's say we have multiple structs that hold various kinds and amounts of text: a
// NewArticle struct that holds a news stroy filed in a particular location and a Tweet that can
// have at most 280 characters along with metadata that indicates whether it was a new tweet,
// retweet, or a reply to another tweet.
// We want to make a media aggregator library that can display summaries of data that might be
// stored in a NewArticle or Tweet instances. To do this, we need a summary from each typem and we
// need to request that summary by calling a summarize method on an instance.
// This example shows the definition of Summary trait that expresses this behavior.
//
// A Summary trait that consists of the behavior provided by summarize method :
pub trait Summary {
    fn summarize(&self) -> String;
}
// Here we declare a trait using the trait keyword and then the trait's name, which is Summary in
// this case. Inside the curly brackets, we declare the method signatures that describes the
// behaviors of the types that implement this trait, which in this case is fn summarize(&self).
// After the method signature, instead of providing an implementation within curly brackets, use use
// a semicolon. Each type implementing this trait must provide its own custom behavior for the body
// of the method. The compiler will enforce that any type that has the Summary trait will have the
// method summarize defined with this signature exactly.
// A trait can have multiple methods in its body : the method signatures are listed one per line and
// each line ends in a semicolon.
//
//
// Implementing a Trait on a Type :
// Now that we've defined the desired behavior using the Summary trait , we can implement it on the
// types in our media aggregator. The next example shows an implementation of the Summary trait on
// the NewsArticle struct that uses the headline, the author, and the location to create the return
// value of summarize.
// For the Tweet struct, we define summarize as the username followed by the entire text of the
// tweet, assuming that tweet content is already limited to 280 characters.
//
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Implementing the Summary trait on the NewsArticle and Tweet types :

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

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
}
// Implementing a Trait on a type is similar to implementing regular methods. The difference is that
// after impl, we put the trait name that we want to implement, then we use the for keyword, and
// then specify the name of the the type we want to implement the trait for. Within the impl block,
// we put the method signatures that the trait definition has defined . Instead of adding a
// semicolon after each signature, we use curly brackets and fill in the method body with the
// specific behavior that we want the methods of trait to have for the particular type.

// Main function
fn main() {
    // After implementing the trait, we can call the methods on instances of NewsArticle and Tweet in
    // the same way we call regular methods, like this :
}
