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
    // Let's try to call panic! in a simple program :
    panic!("crash and burn!");
    // The call to panic! causes the error message contained in the last two lines . The first line
    // shows our panic message and the place in our source code where the panic occurred .
    // In this case , the line indicated is part of our code , and if we go to that line , we see
    // the panic! macro call . In other cases , the panic! call might be in code that our code calls
    // , and the filename and line number reported by the error message will be someone else's code
    // where the panic! macro is called , not the like of our code that eventually led to the panic!
    // call.
    // We can use the backtrace of the functions the panic! call came from to figure ot the part of
    // our code that is causing the problem .
}
