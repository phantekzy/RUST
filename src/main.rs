// COMMON COLLECTIONS
// Storing UTF-8 Encoded Text with Strings
// What is a String ? :
// We'll first define what we mean by the term 'string'. Rust has only one string type in the core
// language , which is the string slice str that is usually seen in its borrowed form &str.
// We talked about string slices . which are references to some UTF-8 encoded string data stored
// elsewhere . String literals , for example , are stored in the program's binary and are therefore
// string slices .
// The String type , which is provided by Rust's standard library rather than coded into the core
// language , is a growable , mutable , own , UTF-8 encoded string type .
// When Rustaceans refer to "strings" in Rust , they usualy mean the String and the string slice
// &str types , not just one of those types . Although i will talk largely about Strings , both
// types are used heavily in Rust's standard library , and both String and string slices are UTF-8
// encoded .
// Rust's standard library also includes a number of other string types , such as OsString , OsStr ,
// CString , and CStr. Library crates can provide even more ooptions for storing string data . see
// how those names all end in String or Str? They refer to owned and borrowed variants , just like
// the String and or Str.
// This string types can store text in different encodings or be represnted in memory in a diff way.
//
// Main function
fn main() {
    // Creating a New String
    // Many of the same operations available with Vec<T> are available with String as well, starting
    // with the new function to create a string :
    // This line creates a new , empty string called s , which we can then load data to it .
    // Often , we'll have some initial data that we want to start the string with .
    // For that , we use the to_string method, which is available on any type that implements the
    // Display trait , as string literals do :
    let data = "initial contents";
    let _s = data.to_string();
    // The method also works on a literal directly :
    let s = "initial contents".to_string();
    println!("{}", s)
}
