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
fn main() {
    // Using a Struct
    // We create an Instance of that struct
    // The Instance
    let user1 = User {
        username: String::from("Phantekzy"),
        email: String::from("Phantekzy@gmail.com"),
        sign_in_count: 3,
        active: false,
    };
    // Getting specific values from the Struct
    // Changing the email value using the dot notation
    // Cannot change the values because it is immutable
    user1.email = String::from("xxx@email.com");
}
