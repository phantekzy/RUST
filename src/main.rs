// STRUCT VS ENUM
// Enums References
// Declarations and Impl Block
// Command Enum
enum Command {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
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
fn main() {}
