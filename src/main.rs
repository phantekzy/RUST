//THE STACK AND THE HEAP
// OWNERSHIP
// OWNERSHIP RULES
fn main() {
    // - Each value in Rust has a variable that's called its owner.
    // The string RUST is created on the heap
    // The variable s is the official owner of that Data
    let s = String::from("RUST");
    // - There can be only one owner at a time.
    // - When the owner goes out of scope , the value will be dropped.
}
