// COMMON COLLECTIONS
// Updating a String
// Concatenation with the + Operator or the format! Macro
// Main function
fn main() {
    // Often we'll want to combine two existing strings. one way is to use the + operator :
    let s1 = String::from("hello, ");
    let s2 = String::from("word!");
    // Using the + operator to combine two string values into a new String value
    let s3 = s1 + &s2; // note that s1 has been moved here and can no longer be used 
}
