// Using Nested Paths to Clean Up Large use Lists
// if we're using multiple items defined in the same package or same module, listing each item on
// its own line can take up a lot of vertical psace in our files .
// For example , these two use statements we had in the Guessing Game bring items from std into
// scope :
//
// Instead we can use nested paths to bring the same items into scope in one line . We do this by
// specifying the common part of the path , followed by two colons , and then curly brackets arround
// a list of the parts of the paths that differ .
use std::{cmp::Ordering, io}; // Specifying a nested path to bring multiple items with the same
// prefix into scope

// Main function
//
fn main() {}
