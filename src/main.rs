// CUSTOM TYPES
// ENUMS AND PATTERN MATCHING

// Another exemple of an enum in that has a wide
// variety of types embedded in its variants
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// Main function
fn main() {}
