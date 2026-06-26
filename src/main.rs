//THE STACK AND THE HEAP
// OWNERSHIP
// OWNERSHIP RULES
fn main() {
    // - Each value in Rust has a variable that's called its owner.
    // The string RUST is created on the heap
    // The variable s is the official owner of that Data
    let s1 = String::from("RUST"); // s1 OWNS the String
    // - There can be only one owner at a time.
    let s2 = s1; // Ownership is moved from s1 to s2
    // - When the owner goes out of scope , the value will be dropped.
    println!("{}", s2);
}
