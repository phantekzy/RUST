// OPTION TYPE
// The match Control Flow Operator
// Coins Enum
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
// Value in cents function
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// Main function
fn main() {}
