// COMMON COLLECTIONS
// Indexing into Strings
// In many other programming languages, accessing individual characters in a string by referencing
// them by index is a valid and common operation.
// However, if you try to access parts of a String using indexing syntax in Rust, we we'll get an
// error
// Main function
fn main() {
    let s1 = String::from("hello");
    let h = s1[0]; // We get an error !
}
