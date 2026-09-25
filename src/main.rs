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
    // The _ pattern will match any value . By puttting it after out other arms ,
    // The _ will match all the possible cases that aren't specified before it.
    // The () is just the unit value , so nothing will happen in the _ case.
    // As result , we can say that we want to do nothing for all the possible values
    // that we don't list before the _ placeholder .
    // However , the match expression can be a bit wordy in a situation in which
    // we care about only "one" of the cases
}
