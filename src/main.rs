// USING STRUCTS TO STRUCTURE RELATED DATA
// Defining and Instantiating Structs
// User Struct Definition
// Structs are like Objects in Languages like Javascript
// The Blueprint
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Creating a function that returns a User instance with the given email and username
fn build_user(email: String, username: String) -> User {
    User {
        username: username,
        email: email,
        sign_in_count: 3,
        active: true,
    }
}
// Main function
fn main() {
    // Using a Struct
    // We create an Instance of that struct
    // The Instance
    let mut user1 = User {
        username: String::from("Phantekzy"),
        email: String::from("Phantekzy@gmail.com"),
        sign_in_count: 3,
        active: false,
    };
    // Changing the value in the email fied of a user Instance
    user1.email = String::from("xxx@email.com");
}
