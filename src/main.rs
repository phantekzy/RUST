// Chapter 10 : Generic types ,trauts , and lifetimes
// Generic Data Types
// In Enum Definitions
// As we did with structs, we can define enms to hold generic data types in their variants, let's
// take another look at tthe Option<T> enum that the standard library provides :
enum Option<T> {
    Some(T),
    None,
} // The definition should now make more sese to us. A we can see Option<T> is an enum that is
// generic over type T and has two variants: Some, which holds one value of type T, and a None
// variant that doesn't hold any value. By using the Option<T> enum, we can express the abstract
// concept of having an optional value, and because Option<T> is generic, we can use this
// abstraction no matter what the type of the optional value is.
// Enums can use multipls generic types as well. The definition of the Result enum that we used in
// the chapter 9 is one example :
enum Result<T, E> {
    Ok(T),
    Err(E),
}
// The result enum is generic over two types , T and E , and has two variants: Ok, which holds a
// value of type T, and Err, which holds a value of type E. This definition makes it convenient to
// use the Result enum anywhere we have an operation that might succeed(Return a value of some type
// T) or fail (Return an error of some type E). In fact, this is what we used to open file in our
// past example , where T was filled in with the type std::fs::File when the file was opened
// successfuly and E was filled in with the type std::io::Error when there were problems opening the
// file.
// Main function
fn main() {}
