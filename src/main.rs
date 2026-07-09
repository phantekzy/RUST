//THE STACK AND THE HEAP
// OWNERSHIP
// REFRENCES AND BORROWING
// DANGLING REFRENCES
// SLICE TYPE
// STRING SLICES
// STRING SLICES ARE SLICES
fn main() {
    let mut s = String::from("Hello world");
    let word = first_word(&s);
    s.clear();
    println!("The first word is {}", word)
}

// first_word function should be written like this
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// Second word function should be written like this
fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i..];
        }
    }
    &s[..]
}

// CONTINUE From STRING LITERALS ARE SLICES
