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
    // 2. Method (Instance-level , takes (&slef))
    fn is_adult(&self) -> bool {
        self.age >= 18
    }
}

fn main() {
    // Calling Associated function (Constructor) -> uses "::"
    let user1 = User::new(String::from("LOTFI"), 24);
}
