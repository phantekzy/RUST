// CUSTOM TYPES
// Enum values
// We can write it in a better way
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from(String::from("::1")));
}
