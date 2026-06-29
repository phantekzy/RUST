//THE STACK AND THE HEAP
// OWNERSHIP
// Memory and Allocation
// Stack Only Data
fn main() {
    // Integers
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    // Difference between Integers and Strings
    let s1 = String::from("hi");
    let s2 = s1;
    println!("s1 = {}, s2 = {}", s1, s2);
}
