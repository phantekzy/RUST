// CUSTOM TYPES
// ENUMS AND PATTERN MATCHING
// Defining an Enum
// Using enums has even more advantages
// We do not have a way to store the actual IP adress data
// We only know what kind it is
// Enum
// We can represent the same concept in a more concise way using an enum
// PS : Enum -> OR  // Struct -> AND
// Another advantage to using enums rather than structs :
// Each variant can have different type and amounts of assiciated data
enum IpAddrKind {
    // IpAddrKind is now a custom data type
    V4(String),
    V6(String),
}
// Main function
fn main() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));
}
