use std::usize;

//THE STACK AND THE HEAP
// OWNERSHIP
// REFRENCES AND BORROWING
// DANGLING REFRENCES
// SLICE TYPE
// STRING SLICES
// STRING SLICES ARE SLICES
fn main() {
    let mut s = String::from("hello word");
    let word = first_word(&s);
    s.clear();
    println!("{}", word)
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
