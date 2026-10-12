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
    // Appending a string slice to a String using the push_str method
    // After these two lines , s will contain "foobar" . The push_str method takes a string slice
    // because we don't necessarily want tot take ownership of the parameter.
    // In this code shows that it would be unfortunate if we weren't avle to use s2 after appending
    // its contents to s1 .
    let mut s1 = String::from("foo"); // Rust allocates space on the heap , copies the letters "foo"
    // into it , and assigns it to s1 .
    // Because we used let mut . we are explicitly telling Rust : "i am going to modify this string
    // later :" s1 owns this data .
    let s2 = "bar"; // Creating a string slice "hard coded"
    // Rust creates a string literal "&str" pointing to the text "bar" baked directly into the
    // program's binary file .
    // s2 is just a lightweight view/pointer . it doesn't own a heap buffer . it just looks at text.
    s1.push_str(s2); // Appending without Stealing 
    // Rust reads the letter from s2"b,a,r" and copies them onto the end of s1's
    // heap memory . now s1 holds "foobar"
    // "push_str" it acepts a borrowed string slice "&str" not ownership of the string .
    println!("s2 is {}", s2);
    // if the push_str method took ownership of s2, we wouldn't be able to print its value on the
    // last line . However, this code works as we'd expect !
}
