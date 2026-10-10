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
// Main function
fn main() {}
