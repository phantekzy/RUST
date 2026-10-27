// Error Handling
// Propagating Errors :
// A shortcut for propagating Errors : the ? Operator

// There is a difference between what the match expresssion in the last example and the ? operator
// do : error values that have the ? operator called on them go through the from function , defined
// in the From train in the standard library , which is used to convert errors from one type into
// another . When the ? operator calls the from function , the error type received is converted into
// the error type defined in the retyrn type of the current function .
// This is useful when a function returns one error type to represent all the ways a function might
// fail , even if parts might fail for many different reasons . As long as each error type
// implements the from function to define how to convert itself to the returned error type , the ?
// operator takes care of the conversion automatically .
//
// In the context of the last example , the ? at the end of File::open call will return the value
// inside an Ok t the variable f . if an error occurs ,the ? operator will return early out the
// whole function and give any Err value to the calling code . The same thing applies to the ? at
// the end of the read_to_string call .
//
// The ? eliminates a lot of boilerplate and makes this function's implementation simpler . We could
// even shorten this code further by chaining method calls immediatlely after the ?

// We've moved the creation of the new String in s to the beginning of the function ; that part
// hasn't changed . instead of creating a variable f , we've chained the call to read_to_string
// directly onto the result of File::open("hello.txt")?. We still have a ? at the end of the
// read_to_string call , and we still return an Ok value containing the username in s when both
// File::open and read_to_string succeed rahter than returning erroors .
// And we can even make it shorter

use std::{fs, io};

// Using fs::read_to_string instead of opening and then reading the file
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
    // Reading a file into a String is a fairly common operation , so Rust provides the convenient
    // fs::read_to_string function that opens the file,create a new String , reads the contents of
    // the file , puts the contents into that String , and returns it .
    // Of course , using fs::read_to_string doens't give us the opportunity to explain all the error
    // handling , so we did it the longer way first .
}

// Main function
fn main() {
    match read_username_from_file() {
        Ok(username) => {
            println!("Success! Welcome, {}", username);
        }
        Err(e) => {
            // The caller handles the error gracefully instead of crashing
            println!(
                "Ah, something went wrong: {}. Using a guest profile instead.",
                e
            );
        }
    }
}
