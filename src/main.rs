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
        // If a pattern matches the value , the code associated with that pattern is executed.
        // If that pattern doesn't match the value , execution continues to the next arm .
        Coin::Penny => 1,    // Match arms
        Coin::Nickel => 5,   // An arm has two parts : a pattern and some code.
        Coin::Dime => 10,    // we separate the patern and the code using the "=>" Operator
        Coin::Quarter => 25, // that separates the pattern and the code to run
    }
    // The code associated with each arm is an expression, and the resulting value of the expression
    // in the matching arm is the value that gets returned for the entire match expression.

    // Curly brackers typically aren't used if the match arm code is short .

    // If we want to run mulitple lines of code in a match arm , we can use curly brackets
}

// Main function
fn main() {}
