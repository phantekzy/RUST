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
    let word = first_word(&s);
    s.clear();
    println!("first word is {}", word);
}

// FIRST WORD FUNCTION
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// SECOND WORD FUNCTION
fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i..];
        }
    }
    &s[..]
}
