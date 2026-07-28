// ENUM
// Storing the data in Structs
// Enums are OR
//
// We can represent the same concept in a more concise way using
// just an enum !
enum IpAddKind {
    V4,
    V6,
} // IpAddKind is now a costum Data type we can use it elsewhere in our code 
// Storing the data in Struct ipAddr
// Struts are AND
struct IpAddr {
    kind: IpAddKind,
    address: String,
}
// Enum values
// Main function
fn main() {
    let four = IpAddKind::V4;
    let six = IpAddKind::V6;
    // We can call the function with either variants
    route(IpAddKind::V4);
    route(IpAddKind::V6);
    // Calling the struct
    // Home
    let home = IpAddr {
        kind: IpAddKind::V4,
        address: String::from("127.0.0.1"),
    };
    // Loopback
    let Loopback = IpAddr {
        kind: IpAddKind::V6,
        address: String::from("::1"),
    };
}

// We can then,for instance , define a function that takes any
// IpAddKind
fn route(ip_kind: IpAddKind) {}
