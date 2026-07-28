// STRUCT VS ENUM
// Enums References
// Declarations and Impl Block
// Command Enum
enum Command {
    Quit,                    // Unit Variant
    Move { x: i32, y: i32 }, // Struct-like variant
    Write(String),           // Tuple-like variant
}
impl Command {
    // Associated function (Type-level , no-self)
    // Returns a default enum state
    fn default_cnd() -> Self {
        Command::Quit
    }
    // Method (Instance-level , takes self as parameter)
    fn log(&self) {
        println!("Command executed");
    }
}
// Main function
fn main() {
    // Calling the associated function
    let cmd1 = Command::default_cnd();
}
