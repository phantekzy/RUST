// Error Handling
// Propagating Errors :

use std::{fs::File, io};

// This function doesn't handle the error . it propagates it upward .
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt");
}
// Main function
fn main() {}
