// COMMON COLLECTIONS
// Updating a String
// A String can grow in size and its contents can change , just like the contents of a Vec<T> , if
// we push more data into it .
// In addition , we can convenently use the + operator for the format! macro to concatenate String
// values .
//

// Main function
fn main() {
    // Appending to a String with push_str and push :
    // We can grow a String by using the push_str mehtod to append a string slice :
    let mut s = String::from("foo");
    // Appending a string slice to a String using the push_str method
    s.push_str("bar");
    // After these two lines , s will contain "foobar" . The push_str method takes a string slice
    // because we don't necessarily want tot take ownership of the parameter.
    // In this code shows that it would be unfortunate if we weren't avle to use s2 after appending
    // its contents to s1 .
}
