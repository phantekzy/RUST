// ENUM
// Storing the data in Structs
// Enums are OR
//
// There is another advantage to using an enum rather than Struct
// Each variant can have different type and amounts of associated data .
enum _IpAddKind {
    V4(u8, u8, u8, u8),
    V6(String),
} // IpAddKind is now a costum Data type we can use it elsewhere in our code 
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}
// Main function
fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopbacl = IpAddr::V6(String::from("::1"));
    println!("Home : {:?}", home);
}
