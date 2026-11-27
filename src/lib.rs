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
// need to request that summary by calling a summarize method on an instnace.
