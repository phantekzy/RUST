//THE STACK AND THE HEAP
// OWNERSHIP
// REFRENCES AND BORROWING
// MUTABLE REFRENCES
fn main() {
    let mut s = String::from("BORROWING");
    let mut r = &mut s;
    // We can have only one mutable reference
    let mut r1 = &mut s;
    println!("{}, {}", r, r1);
}
