//THE STACK AND THE HEAP
// OWNERSHIP
// REFRENCES AND BORROWING
// DANGLING REFRENCES
// SLICE TYPE
fn main() {
    let mut s = String::from("Hello World");
    let _word = first_word(&s);

    s.clear(); // Clear empties the String 
} // word still has the value 5 but there is no more strings 
// that we can use the value 5 with . word is totaly now invalid

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
