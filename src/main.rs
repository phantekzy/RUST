//THE STACK AND THE HEAP
// OWNERSHIP
// REFRENCES AND BORROWING
// DANGLING REFRENCES
// SLICE TYPE
// STRING SLICES
fn main() {
    let s = String::from("Hello");
    // If we want to start at the first index , we can drop the value before two periodes
    // Both are equqls
    let slice = &s[0..2];
    let slice = &s[..2];
}
