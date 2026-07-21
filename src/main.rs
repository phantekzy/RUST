// Store a Reference in a Struct without a specifying lifetimes
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

// Main Function
fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "sawthemovie",
        active: true,
        sign_in_count: 2,
    };
}
