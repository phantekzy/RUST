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
// In Bigger programs , bringing many items into scope from the same package or module using nested
// paths can reduce the number of separate use statements needed by a lot ! .
// We can use a nested path at any level in a path , which is useful when combining two use
// statements that share a subpath.
// For example in this listing it shows two use statements : one that brings std::io into scope and
// one that brings std::io:Write into scope .

// The common part of these two paths is std::io , and that's the complete first path .
// To merge these two paths into one statement , we can use self in the nested path :
use std::io::{self, Write};
// Main function
//
fn main() {}
