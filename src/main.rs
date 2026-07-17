//THE STACK AND THE HEAP
// BORROWING AND OWNERSHIP IN RUST
// SLICE TYPE
// STRING SLICES
// STRING lITERALS ARE SLICES

// MAIN FUNCTION
fn main() {
    //let mut s = String::from("Hello world");
    // S here is a String literal because it is hard coded and stored inside the binary
    let s = "Hello world"; // Type of s is &str pointing to the binary location
    let first = first_word(s);
    let second = second_word(s);
    println!("first word is {} and the second word is {}", first, second);
}

// FIRST WORD FUNCTION
// String slices as parameters
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// SECOND WORD FUNCTION
fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i..];
        }
    }
    &s[..]
}
