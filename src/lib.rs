// Defining a Trait :
// A type's behavior consists of the methods we can call on that type. Differnt types share the same
// behavior if we can call the same methods on all of those types. Trait definitions are a way to
// group method signatures together to define a set of behavior necessary to accomplish some
// purpose.
// For example, let's say we have multiple structs that hold various kinds and amounts of text: a
// NewsArticle struct that holds a news story filed in a particular location and a Tweet that can
// have at most 280 characters along with metadata (data about data) that indicates whether it was a
// new tweet , a retweet  , or a reply to another tweet.
// We want to make a media aggregator library that can display summaries of data that might be
// stored in a NewsArticle or Tweet instance. To do this, we need a summary from each type, and we
// need to request that summary by calling a summarize method on an instance.
pub trait Summary {
    fn summarize(&self) -> String; // A Summary trait that consists of the behavior provided by a
    // summarize method
}
// Here we declare a trait using the trait keyword and then the trait's name, which is Summary in
// this case. Inside the curly brackets, we declare the method signatures that describe the
// behaviors of the types that implement this trait, which in this case is fn summarize(&self) ->
// String
// After the method signature, instead of providing an implementation within curly brackets, we use
// a semicolon. Each type implementing this trait must provide its own custom behvaior for the body
// of the method. The compiler will enforce that any type that has the Summary trait will have the
// method summarize defined with this signature exactly.
// A trait can have multiple methods in its body: the method signatures are listed one per line and
// each line ends with a semicolon.
//
// Implementing a Trait on a Type :
// Now that we've defined the desired behavior using the Summary trait, we can implement it on the
// types in our media aggregator.
//
// NewsArticle Struct :
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by{} ({})", self.headline, self.author, self.location)
    }
} // Implementing the Summary trait on the Tweet type

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
