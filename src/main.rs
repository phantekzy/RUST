use std::fmt::Debug;

// COMMON COLLECTIONS
// Updating a String
// Concatenation with the + Operator or the format! Macro
// Main function
fn main() {
    // If we need to concatenate multiple strings , the behavior of the + operator gets unwidldy :
    // at this point , s will be "tic-tac-toe" . With all of the + and " characters, it's difficult
    // to see what's going on .
    // For more complicated string combining , we can use the format! macro :
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
    // this code also sets s to "tic-tac-toe". The format! macro works in the same way as println!(""),
    // but instead of printing the output to the screen it retruns a String with the contents.
    // The version of the code using format! is much easier to read and doesnt take ownership of any
    // of its parameters
}
