// CUSTOM TYPES
// ENUMS AND PATTERN MATCHING
// Defining an Enum
enum IpAddrKind {
    // IpAddrKind is now a custom data type
    V4,
    V6,
}
// Main function
fn main() {
    // Enum Values
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}
