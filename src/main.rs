// The _ Placeholder
// Rust also has a pattern we can use when we don't want to list all possible values .
// For example , a u8 can have valid values of 0 thriygh 255 . If we only care about the values
// 1,3,5 and 7 , we dont want tot have to list out 0,2,4,6,8,9 all the way to 255
// Fortunately , we don't have to .
// Main function
fn main() {
    let some_u8_value = 0u8; // Initializing 0 as an explicit 0-bit unsigned integer.
    match some_u8_value {
        1 => println!("one"),
        3 => println!("thre"),
        5 => println!("five"),
        7 => println!("sever"),
        _ => (),
    }
}
