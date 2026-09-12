// CUSTOM TYPES
// Message enum whose variants each store different amounts and types of values
enum Message {
    // This Enum has four variants
    Quit,                       // Quit has no data associated with it at all
    Move { x: i32, y: i32 },    // Move includes an anonymous struct iniside of it
    Write(String),              // write includes a single String
    ChangeColor(i32, i32, i32), // ChangeColor includes three i32 values
}
// Definingh an enum of variants such as the one before is similar to defining
// different kinds of struct definitions , except the enum doesn't use the struct keyword
// and all the variants are grouped together under the message type

struct QuitMessage; // Unit Struct 

fn main() {}
