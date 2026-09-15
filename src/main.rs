// OPTION TYPE
// Main function
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // Option<T> and T are different types
    // Rust doesn't understand how to add an i8 and an Option<T> , because they are different types
    let sum = x + y;
}
