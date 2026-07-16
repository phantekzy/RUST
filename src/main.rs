use std::io::Bytes;

//THE STACK AND THE HEAP
// OWNERSHIP
// REFRENCES AND BORROWING
// DANGLING REFRENCES
// SLICE TYPE
// STRING SLICES
fn main() {
    let s = String::from("Hello world");
    let first = first_word(&s);
    println!("the first word is {}", first);
}

// First word filter
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// Second word Filter
fn second_word(s: &String) -> &str {
    let Bytes = s.as_bytes();
    for (i, &item) in Bytes.iter().enumerate() {
        if item == b' ' {}
    }
}
