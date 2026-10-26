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
    let f = File::open("xxx.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
