// Enumerations
// Enums allows us to define a type by enumerating its possible values.
// Defining an enum

// Difference between a struct and enum
// Struct means AND
// Enums means OR

// IP ADRESS ENUM
enum IpAddrKind {
    // IpAddrKind is now a costum data TYpe
    V4,
    V6,
}
// Stock the enums
// We define the struct
struct IpAddr {
    kind: IpAddrKind,
    adress: String,
}
// Main Function
fn main() {
    // Enum Values
    // We use the double colon to separate to call the values and separate the two
    let _v4 = IpAddrKind::V4;
    let _v6 = IpAddrKind::V6;
    // And we can call this function with either variant
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr
}
// IpAddrKind::v4 and IpAddrKind::v6 are of the same type : IpAddrKind
// We can then , for instance , define Functions that takes any IpAddrKind
fn route(ip_kind: IpAddrKind) {}
