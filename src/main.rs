// STRUCT VS ENUM
// HOW TO USE A STRUCT
struct User {
    username: String,
    age: u32,
}

// Implementation of User struct
impl User {
    // 1. Associated Function (Type-level , so-self)
    fn new(username: String, age: u32) -> Self {
        User { username, age }
    }
}

fn main() {}
