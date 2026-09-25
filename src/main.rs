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
    //
    // Instead, we could write this in a shorter way using if let :
    if let Some(3) = some_u8_value {
        println!("three");
    }
    // The syntax if let takes a pattern and an expression separated by an equal sign.
    // It works the same way as a match , where the expression is given to the match and the pattern
    // is its first arm .
    // Using if let means less typing , less indentation , and less boilerplate code .
    //
    // However , we lose the exhaustive checking that match enforces.
    // Chossing between match and if let depends on what we are doing in our particular situation
    // and wheter gaining conciseness in an appropriate trade-off for losing exhaustive chacking .
}
