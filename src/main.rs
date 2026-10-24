use std::fs::File;

// Error handling
// Recoverable Errors with Result :
// Most errors aren't serious enough to require the program to stop entirely.
// Sometimes , when a function fails , it's for a reason that we can easily interpret and respond to
// . For example , if we try to open a file and that operation fails because the file doesn't exist
// , we might want to create the file instead of terminated the process .
enum Result<T, E> {
    OK(T),
    Err(E),
}
// The T and E are generic type parameters : we'll discuss generics in more details later , what we
// need to know right now is that T represents the type of the value that will be returned in a
// success case within the OK variant , and E represents the type of error that will be returned in
// a failure case within the Err variant .
// Because Result has these generic type parameters , we can use the Result type and the functions
// that the standard library has defined on it in many different situations where the successful
// value and error value we want to return may differ .
// Main function
fn main() {
    // Let's try to open a file :
    let f = File::open("hello.txt"); // Opening a file 
}
