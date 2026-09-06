// CUSTOM TYPES
// Enum values
// We can write it in a better way
// if we wanted t o store V4 addresses as four u8 values but still express v6 addresses as a string
// We wouldn't be able to with a struct
enum IpAddr {
    // We attach data to each variant of the enum directly, so there is no need for an extra struct
    V4(String),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from(String::from("::1")));
}
