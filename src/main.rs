// CUSTOM TYPES
// Enum values

struct Ipv4Addr {
    // -- snip
}

struct Ipv6Addr {
    // -- snip
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

fn main() {}
