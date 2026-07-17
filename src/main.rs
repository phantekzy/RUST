//THE STACK AND THE HEAP
// OWNERSHIP
// REFRENCES AND BORROWING
// DANGLING REFRENCES
// SLICE TYPE
// STRING SLICES

// MAIN FUNCTION
fn main() {
    let s = String::from("Hello world");
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
