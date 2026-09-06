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

// This code illustrates that you can put any kind of data inside an enum variant : strings ,
// numeric types or structs ,  We can even include another enum

fn main() {}
