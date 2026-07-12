//THE STACK AND THE HEAP
// OWNERSHIP
// REFRENCES AND BORROWING
// DANGLING REFRENCES
// SLICE TYPE
// STRING SLICES
fn main() {
    let s = String::from("Hello world");
    // The slice data structure , stores the starting position
    // and the length of the slice
    let hello = &s[..5];
    let world = &s[6..];
    // To take the entire string
    let all = &s[..];
    println!("{}", hello);
    println!("{}", world);
    println!("{}", all);
}
