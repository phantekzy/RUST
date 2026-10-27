// Error Handling
// Propagating Errors :
// A shortcut for propagating Errors : the ? Operator

use std::{fs::File, io};

// This function doesn't handle the error . it propagates it upward .
fn read_username_from_file() -> Result<String, io::Error> {}
// Main function
fn main() {}
