// Error handling
// Rust groups errors into two major categories : "recoverable" and "unrecoverable errors".
// For recoverable error , such as a file not found error , it's resonable to report the problem to
// user and retry the operation .
// Unrecoverable errors are always symptoms of bugs , like trying to access a location beyond the
// end of an array .
// Most languages don't distinguish between these two kinds of errors and hanlde both in the same
// way , using mechanisms such as exceptions . Rust doesn't have exceptions .
// Main function
fn main() {}
