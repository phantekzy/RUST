// Error Handling
// Propagating Errors :
// The ? Operator Can Only Be Used in Functions That Return Result :
// The ? operator can only be used in Functions that have a return type of Result , because it is
// defined to work in the same way as the match expression we defined in the last example .
// The part of the match  that requires a return type of Result is return Err(e) , so the return
// type of the function must be a Result to be compatible with this return .

use std::{error::Error, fs::File};

// Main function
// Let's look at what happens if we use the ? operator in the main fnction , which we'll recall
// has a return type of () :
// error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or anothe
//r type that implements `FromResidual`)
//This error points out that we're only allowed to use the ? operator in a function that returns
//Result<T,E> . When we are wrting code in a function that doesn't return Result<T,E> ,and we
//want to use the ? try operator when we call other functions that return Result<T,E> , we have
//two choices to fix this problem.
//One technique is to change the return type of our function to be Result <T,E> if we have no
//restrictions preventing that . The other technique is to use a match or one of the Result<T,E>
//methods to handle the Result<T,E> in whatever way is appropriate .
//The main function is special and there are restrictions on what its retrun type must be  . One
//valid return type for main is () . and conveniently , another return type is Result<T,E> :.
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}
