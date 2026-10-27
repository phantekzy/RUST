// Error Handling
// Propagating Errors :
// A shortcut for propagating Errors : the ? Operator

use std::{
    fs::File,
    io::{self, Read},
};

// A function that returns error to the calling code using the ? operator
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
// Main function
fn main() {}
