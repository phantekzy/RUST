// CUSTOM TYPES
// Enum values
// We can create instances of each of the two variants of IpAddrkind
enum IpAddrkind {
    // Custom type
    V4,
    V6,
}
struct IpAddr {
    // Struct
    kind: IpAddrkind,
    address: String,
}
fn main() {
    // IP Versions
    let four = IpAddrkind::V4;
    let six = IpAddrkind::V6;
    // Home Ip Address v4
    let _home = IpAddr {
        kind: four,
        address: String::from("127.0.0.1"),
    };
}
