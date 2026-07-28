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

enum IpAddr {
    V4(String),
    V6(String),
}
// Main function
fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopbacl = IpAddr::V6(String::from("::1"));
}
