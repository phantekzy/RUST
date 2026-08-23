// CUSTOM TYPES
// ENUMS AND PATTERN MATCHING

// Another exemple of an enum in that has a wide
// variety of types embedded in its variants

// A Message enum whose variants each store different amounts and types values
// This enum has four variants with different types :
//  - Quit has no data associated with it at all.
//  - Move includes an anonymouse struct inside it.
//  - Write includes a single String.
//  - ChangeColor includes three i32 values.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// Main function
fn main() {}
