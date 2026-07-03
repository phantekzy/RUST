//THE STACK AND THE HEAP
// OWNERSHIP
// REFRENCES AND BORROWING
// MUTABLE REFRENCES
fn main() {
    let mut s = String::from("BORROWING");
    let r = &mut s;
    // We can have only one mutable reference
    // The benefit of having this restriction is that Rust can prevent
    // data races at compile time .
    println!("{} ", r);
}
