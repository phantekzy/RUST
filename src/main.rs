// COMMON COLLECTIONS
// Indexing into Strings
// Internal Representaion :
// A String is a wrapper over a Vec<u8>.
// Main function
fn main() {
    let len = String::from("hola").len();
    println!("{}", len); // In this case , len will be 4, which means the vector storing the string "hola" is 4 bytes long . Each of these letters takes 1 byte when encoded in UTF-8. But what about this
    // following line ?.
    let len2 = String::from("你好").len(); // Hello in Chinesse.
    println!("{}", len2); // Asked how long the string is , you might say its 3 , Rust's answer is
    // 6
}
