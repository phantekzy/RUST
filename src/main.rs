// OPTION TYPE
// The match Control Flow Operator
// Coins Enum
// An enum and a match expression that has
// the variants of the enum as its patterns
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
// Value in cents function
fn value_in_cents(coin: Coin) -> u8 {
    // Match expression
    match coin {
        // Patterns
        Coin::Penny => 1,  // Match arms
        Coin::Nickel => 5, // An arm has two parts : a pattern and some code.
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
    // this seems very similar to an expression used with if ?
    // But there is a big difference :
    // With If : the expression needs to return a Bollean value ,
    // but here with the match expresssion , it can be any type.
    // The type of coin in this example is the Coin enum
}

// Main function
fn main() {}
