// Error handling
// Rust groups errors into two major categories : "recoverable" and "unrecoverable errors".
// For recoverable error , such as a file not found error , it's resonable to report the problem to
// user and retry the operation .
// Unrecoverable errors are always symptoms of bugs , like trying to access a location beyond the
// end of an array .
// Most languages don't distinguish between these two kinds of errors and hanlde both in the same
// way , using mechanisms such as exceptions . Rust doesn't have exceptions . Instead , it has the
// type Result<T,E> for recoverable errors and the panic! macro that stops exections when the
// program encounters an unrecoverable error.
//
// Unrecoverable Errors with panic! :
// Sometimes , bad things happen in our code , and there's nothing we can do about it , In these
// case ,  Rust has the panic! macro . When the panic! macro executes , our program will print a
// failure message , unwind and clean up the stack , and then quit . This most commonly occurs when
// a bug of some kind has been detected and it's not clear to the programmer how to handle the error
//
// Main function
fn main() {
    // Using a Panic! Backtrace
    // let's look at another example to see what it's like when a panic! call comes from a library
    // because of a bug in our code instead of from our code calling the macro directly .
    let v = vec![1, 2, 3];
    v[99]; // Attempting to access an element beyond the end of a vector , which will cause a call to
    // panic !
    // Here , we're Attempting to access the 100th element of our vector (which is index 00) , but
    // it has only 3 elements . In  this situation , Rust will panic . Using[] is supposed to return
    // an element , but if we pass an invalid index , there's no element that Rust could return here
    // that would be correct .
    // Other languages like C , will attempt to give us exactly what we asked for in this situation
    // , even though it isn't what we want : we'll get whatever is at the location in memory that
    // would correspond to that element in the vector, even though the memory doesn't  belong to the
    // vector .
}
