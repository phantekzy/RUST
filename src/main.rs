// Concise Control Flow with if let
// The if let syntax lets us combine if and let into a less verbose way to handle values that match
// one pattern while ignoring the rest .

//  Main Function
fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
}
