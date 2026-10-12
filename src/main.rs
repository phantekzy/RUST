use std::fmt::Debug;

// COMMON COLLECTIONS
// Updating a String
// Concatenation with the + Operator or the format! Macro
// Main function
fn main() {
    // If we need to concatenate multiple strings , the behavior of the + operator gets unwidldy :
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + s2 + "-" + s3;
    println!("{}", s);
}
