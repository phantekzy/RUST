// CUSTOM TYPES
// ENUMS AND PATTERN MATCHING
// Defining an Enum
// Using enums has even more advantages
// We do not have a way to store the actual IP adress data
// We only know what kind it is
// Enum
enum IpAddrKind {
    // IpAddrKind is now a custom data type
    V4,
    V6,
}
// Struct
struct IpAddr {
    kind: IpAddrKind,
    adress: String,
}
// Main function
fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        adress: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        adress: String::from("::1"),
    };
}
