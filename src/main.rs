// COMMON COLLECTIONS
// Methods for Iteratin over Strings :
// Fortunately , we can accesss elements in a string in other ways .
// If we need to perform operations on individual Unicode scalar values ,
// the best way to do so is to use the chars method . Calling chars on "नमस्ते"separates out and
// returns 6 values of type char , and we can iterate overt the result to access each element :
//
// Main function
fn main() {
    // Chars methode :
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    // Bytes methode :
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
