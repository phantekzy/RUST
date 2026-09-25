// Concise Control Flow with if let
// The if let syntax lets us combine if and let into a less verbose way to handle values that match
// one pattern while ignoring the rest .

//  Main Function
fn main() {
    let some_u8_value = Some(0u8);
    // A match that only cares about executing code when the value is Some(3)
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // We want t o do sonething with the Some(3) match but do nothing with any other Some<u8> value
    // or the None value .
    // To satisfy the match expression , we have to add _ => () after procession just one variant ,
    // which is a lot of boileerplate code to add .
}
