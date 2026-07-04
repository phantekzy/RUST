//THE STACK AND THE HEAP
// OWNERSHIP
// REFRENCES AND BORROWING
// DANGLING REFRENCES
// SLICE TYPE
fn main() {
    // I HAD TO REVIEW LOOPS
    // Tracking the Index with Enumerate
    let bytes = b"hello"; // A byte slice of the string
    for (index, byte) in bytes.iter().enumerate() {
        println!("index {} holds byte {}", index, byte);
    }
}
