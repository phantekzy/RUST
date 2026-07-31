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

// build_user that takes an email and username and returns User instance
// Using the Field init shorthand
fn build_user(email: String, username: String) -> User {
    // We use the Field init shorthand when the
    // variables and the fields have the same name
    User {
        username, // Removed the username :
        email,    // Removed the email :
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
