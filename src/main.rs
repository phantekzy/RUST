// CUSTOM TYPES
// Enum values
// We can create instances of each of the two variants of IpAddrkind
enum IpAddrkind {
    V4,
    V6,
}
struct IpAdr {
    kind: IpAddrkind,
    address: String,
}
fn main() {}
