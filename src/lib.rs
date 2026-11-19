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
