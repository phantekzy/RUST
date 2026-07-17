//THE STACK AND THE HEAP
// BORROWING AND OWNERSHIP IN RUST
// SLICE TYPE
// STRING SLICES
// STRING lITERALS ARE SLICES

// MAIN FUNCTION
fn main() {
    let my_string = String::from("Hello World");
    // First word works on slices of strings
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";
    // First word works on slices of string literals
    let word = first_word(my_string_literal);
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
