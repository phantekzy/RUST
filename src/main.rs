// Error Handling
// Propagating Errors :
// A shortcut for propagating Errors : the ? Operator

use std::{
    fs::File,
    io::{self, Read},
};

// This function doesn't handle the error . it propagates it upward .
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
// Main function
fn main() {}
