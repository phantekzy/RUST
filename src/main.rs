use std::fmt::Display;
fn main() {
    // Tuples
    // Distructuring
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let first = x.0;
    let second = x.1;
    println!("the first value of x tuple is {}", first);
    println!("the second value of x tuple is {}", second);
}
