// ENUM
// Storing the data in Structs
// Enums are OR
//
// There is another advantage to using an enum rather than Struct
// Each variant can have different type and amounts of associated data .

// How the Standard Libray Defines IpAddr::
// IP version 4 Address
// Standard Libray type are growable data
struct Ipv4Addr {
    // -- snip --
}
// IP version 4 Address
struct Ipv6Addr {
    // -- snip --
}
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// Main function
fn main() {}
