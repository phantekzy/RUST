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

// Note that the variants of the enums are namespaced under its indentifier.
// We use a double colon to separate the two.
// Both values IpAddrKind::V4 and IpAddrKind::V6 are of the same type : IpAddrKind.
