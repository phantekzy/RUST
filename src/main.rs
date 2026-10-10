// Providing New Names with the as Keyword
// There is another sollution to the problem of bringing two types of the same name into the same
// scope with use : after the path , we can specify as and a new local name , or alias , for the
// type .
use std::fmt::Result;
// Renaming a type when it's brought into scope with the as Keyword
use std::io::Result as IoResult;
fn function1() -> Result {
    // --snip --
}
fn function2() -> IoResult<()> {
    // --snip --
}
// In the second use statement , we chose the new name IoResult for the std::io::Result type , which
// won't conflict with the Result from std::fmt that we've also brought into scope
// Both here are considered idiomatic , so the choice is up to you .

// Main function
fn main() {}
