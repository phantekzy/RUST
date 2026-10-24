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
    // How do we know File::open returns a Result ?? We could look at the standard library API
    // documentation , or we can ask the compiler! if we give f a type annotation that we know is
    // not a return type of the function and then try to compile the code , the compiler will tell
    // us that the types don't match . The error message will then tell us what the type of f is .
    // let's try it ! we know that the return type of File::open isn't type u32 , so let's hange the
    // let f statement :
    let f = File::open("hello.txt"); // Opening a file 
    // Attempting to compile gives us a mismatch type error
    // This tells us the return type of the File::open function is a Result<T,E> .
    // The generic parameter T has been filled in here with the type of the success value use std::fs::File; Which is a file handle .
    // the type of E used in the error value is std::io::Error.
    // This return type means the call to File::open might succeed and return a file fandle that we
    // can read from or  write to . The function call also might fail : for example the file might
    // not exist , or we might not have permission to access the file .
    // The file::open function needs to have a way to tell us whether it succeeded or failed and at
    // the same time give us either the file handle or error information .
    // This information is exactly what the Result enum conveys.
    // In the case where File::open succeeds , the value in the variable f will be an instance of OK
    // that contains a file handle . in the case where it fails , the value in f will be an instance
    // of Err that contains more information about the kind of error that happned .
    // We need to add to the code to take diffrent actions depending on the value File::open returns
}
