//THE STACK AND THE HEAP
// OWNERSHIP
// REFRENCES AND BORROWING
// DANGLING REFRENCES
// SLICE TYPE
// STRING SLICES
fn main() {
    let s = String::from("Hello");
    let len = s.len();
    let slice = &s[3..len];
    // We can drop the trailing number to get the last byte
    let slice = &s[3..];
}
