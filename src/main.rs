//THE STACK AND THE HEAP
// OWNERSHIP
// REFRENCES AND BORROWING
// DANGLING REFRENCES
// SLICE TYPE
// STRING SLICES

// MAIN FUNCTION
fn main() {
    let s = String::from("Hello world");
    let first = first_word(&s);
    let second = second_word(&s);
    println!("first word is {}", first);
    println!("second word is {}", second);
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
