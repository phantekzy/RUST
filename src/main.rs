// CUSTOM TYPES
// ENUMS AND PATTERN MATCHING
// How the standard Library defines IpAddr:
// It has the exact enum variants that we've defined and used
// But it embeds the address data inside the variants in the form
// of two different structs , which are defined differently for each variant :

// IP version 4
struct Ipv4Addr {}

// IP version 6
struct Ipv6Addr {}
// Main function
fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
}
