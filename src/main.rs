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
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// Main function
fn main() {}
