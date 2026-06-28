//THE STACK AND THE HEAP
// OWNERSHIP
// String type
// Why strings can be mutated but not litterals ?
// Memory and Allocation
// Ways That Variables and Data Interact : Move
fn main() {
    let x = 5;
    let y = x;
    println!("{}", y)
    // Integers have a fixed size that is known in compile time , they live in stack
    // Rust give integers a special property called Copy trait
}
