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
// of the method.
// Main function
fn main() {}
