//THE STACK AND THE HEAP
// OWNERSHIP
// REFRENCES AND BORROWING
// DANGLING REFRENCES
// SLICE TYPE
fn main() {}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // as_bytes when it is in a Runtime

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // b when it is hard coded
            return i;
        }
    }
    s.len()
}
