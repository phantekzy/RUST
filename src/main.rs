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
// Main function
fn main() {}
