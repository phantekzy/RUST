// Error Handling
// Propagating Errors :
// The ? Operator Can Only Be Used in Functions That Return Result :
// The ? operator can only be used in Functions that have a return type of Result , because it is
// defined to work in the same way as the match expression we defined in the last example .
// The part of the match  that requires a return type of Result is return Err(e) , so the return
// type of the function must be a Result to be compatible with this return .

use std::fs::File;

// Main function
fn main() {
    // Let's look at what happens if we use the ? operator in the main fnction , which we'll recall
    // has a return type of () :
    let f = File::open("hello.txt")?;
    // error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or anothe
    //r type that implements `FromResidual`)
    //This error points out that we're only allowed to use the ? operator in a function that returns
    //Result<T,E> . When we are wrting code in a function that doesn't return Result<T,E> ,and we
    //want to use the ? try operator when we call other functions that return Result<T,E> , we have
    //two choices to fix this problem
}
