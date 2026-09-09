// CUSTOM TYPES
// Message enum whose variants each store different amounts and types of values
enum Message {
    // This Enum has four variants
    Quit,                       // Quit has no data associated with it at all
    Move { x: i32, y: i32 },    // Move includes an anonymous struct iniside of it
    Write(String),              // write includes a single String
    ChangeColor(i32, i32, i32), // ChangeColor includes three i32 values
}

fn main() {}
