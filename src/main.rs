use std::{
    fs::File,
    io::{self, Read},
};

// Error Handling
// Propagating Errors :
// When we are writing a function whose implementation calls something that might fail , instead of
// handling the error withing this function , we can return the error to the calling code so that it
// can decide what to do .
// This is known as "Propagating"the error and gives more control to the calling code , where there
// mmight me more information or logic that dictates how the error should be handled than what you
// have available in the context of your code .
fn main() {}
// For example this code shows a function that reads a username from a file . If the file doen't
// exist or can't be read , this function will return those errors to the code that called this
// function :
// A function that returns errors to the calling code using match
// REDO THIS FUNCTION !!
fn read_username_from_file() -> Result<String, io::Error> {
    // Opening the file
    // This attempts to open the file name hello.txt
    let f = File::open("hello.txt");
    // Because File::open can fail , it returns its own Result
    // Handling the file Open Result
    // We use a match expression to inspect the result of File::open
    let mut f = match f {
        Ok(file) => file, // extract the actual file handle and store in the mut f .
        Err(e) => return Err(e), // Immediately exit the function using return Err(e) , handling the
                           // error back to the caller .
    };
    // Creating the Buffer String
    let mut s = String::new(); // Before we can pull text out of the file , we need a mutable string
    // container in memory s to hold the incoming bytes .
}
