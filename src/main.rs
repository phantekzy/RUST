// STRUCT VS ENUM
// HOW TO USE A STRUCT
struct User {
    username: String,
    age: u32,
}

fn main() {
    // 1. Creating (instantiating) a struct using { }
    let user1 = User {
        username: String::from("Lotfi"),
        age: 29,
    };
    // 2. Reading fields using the dot operator .
    println!("Name : {}", user1.username);
    println!("Age : {}", user1.age);
}
