//THE STACK AND THE HEAP
// OWNERSHIP
// REFRENCES AND BORROWING
// DANGLING REFRENCES
// SLICE TYPE
// STRING SLICES
fn main() {
    let string_one = String::from("Hello world");
    let one = first_word(&string_one);
    let two = second_word(&string_one);
    // PRINTS
    println!("{}", one);
    println!("{}", two)
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
