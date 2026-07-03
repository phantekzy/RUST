//THE STACK AND THE HEAP
// OWNERSHIP
// REFRENCES AND BORROWING
// MUTABLE REFRENCES
fn main() {
    let mut s = String::from("BORROWING");
    // New scope
    {
        let r = &mut s;
        println!("{} ", r);
    };
    // r goes out of the scope , and here we can make a new reference with no problems
    // A similar rule exists for combining mutable and immutable references
    // Cannot have an mutable reference if we have immutable ones.
    let r1 = &s; // No problem
    let r2 = &s; // No problem
    let r3 = &mut s; // ERROR

    println!("{},{} and {}", r1, r2, r3);
}
