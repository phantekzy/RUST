//THE STACK AND THE HEAP
// OWNERSHIP
// REFRENCES AND BORROWING
// DANGLING REFRENCES
// SLICE TYPE
// STRING SLICES
fn main() {
    let s = String::from("Hello");
    let len = s.len();
    // To get the entire string
    let slice = &s[0..len];
    // We also can drop both values to get the entire string
    let slice = &s[..];
}
