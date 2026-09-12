// CUSTOM TYPES
// Defining Methods on enums
// Message enum whose variants each store different amounts and types of values
enum Message {
    // This Enum has four variants
    Quit,                       // Quit has no data associated with it at all
    Move { x: i32, y: i32 },    // Move includes an anonymous struct iniside of it
    Write(String),              // write includes a single String
    ChangeColor(i32, i32, i32), // ChangeColor includes three i32 values
}
// impl a method for the enum
impl Message {
    fn call(&self) {
        // Method body would be defined here
    }
}
// Definingh an enum of variants such as the one before is similar to defining
// different kinds of struct definitions , except the enum doesn't use the struct keyword
// and all the variants are grouped together under the message type

// QuitMessage Struct is a unit struct
struct QuitMessage; // Unit Struct 

// MoveMessage
struct MoveMessage {
    x: i32,
    y: i32,
}

// WriteMessage struct is a tuple struct
struct WriteMessage(String);

// ChangeColormessage is also a tuple struct
struct ChangeColorMessage(i32, i32, i32);

// Main function
fn main() {}
