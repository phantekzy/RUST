//THE STACK AND THE HEAP
// OWNERSHIP
// String type
// Why strings can be mutated but not litterals ?
// Memory and Allocation
// Ways That Variables and Data Interact : Move
fn main() {
    // Integers
    let x = 5;
    let y = x;
    println!("{}", y);
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}
