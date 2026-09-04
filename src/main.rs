// CUSTOM TYPES
// Enum values
// We can create instances of each of the two variants of IpAddrkind
enum IpAddrkind {
    V4,
    V6,
}
fn main() {
    let four = IpAddrkind::V4; // Four and six are of the same type IpAddrkind
    let six = IpAddrkind::V6;
}
// We can then , for instance , define a function htat takes any IpAddrkind
fn route(ip_kind: IpAddrkind) {}
