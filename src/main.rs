// CUSTOM TYPES
// ENUMS AND PATTERN MATCHING
// How the standard Library defines IpAddr:
enum IpAddrKind {
    // Each variant can have different type and amounts of assiciated data
    V4(u8, u8, u8, u8),
    V6(String),
}
// Main function
fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
}
