// Enumerations
// Enums allows us to define a type by enumerating its possible values.
// Defining an enum

// IP ADRESS ENUM
enum IpAddrKind {
    // IpAddrKind is now a costum data TYpe
    V4,
    V6,
}

// Main Function
fn main() {
    // Enum Values
    // We use the double colon to separate to call the values and separate the two
    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;
}
