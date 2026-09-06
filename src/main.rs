// CUSTOM TYPES
// Enum values
// Wide variety of types embedded in this enum variants
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
fn main() {}
