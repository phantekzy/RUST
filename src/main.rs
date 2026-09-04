// CUSTOM TYPES
// Enum values
// We can create instances of each of the two variants of IpAddrkind
enum IpAddrkind {
    V4,
    V6,
}
fn main() {
    let four = IpAddrkind::V4;
    let six = IpAddrkind::V6;
}
