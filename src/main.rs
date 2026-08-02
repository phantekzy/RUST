// USING STRUCTS TO STRUCTURE RELATED DATA
// Defining and Instantiating Structs
// User Struct Definition
// Structs are like Objects in Languages like Javascript
// Using Tuple Structs Without Named Fields to create Different Types
// The Blueprint
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
// We can also define Structs that looks similar to tuples, calles "Tuple Structs"
// Tuple Structs are useful when you want to give the whole tuple a name
// and make the tuple be a different type from other tuples
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

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
    // Creating a new User Instance using some of the values from user1
    let mut user2 = User {
        email: String::from("Ferchouch"),
        username: String::from("heyjude@something.com"),
        sign_in_count: user1.sign_in_count,
        active: user1.active,
    };
    // Using Struct Update Syntax
    // To write less code
    // the ".." Syntax specifies that the remining fields not explicitly set should have
    // the same value as the fields in the given instance
    let user3 = User {
        email: String::from("anotherex@exmple.com"),
        username: String::from("anotherusername"),
        ..user1
    };
    // Using Struct Update Syntax to set new email and username values for a User
    // instance but user the rest of the value from the fields of the instance
    // in the user1 variable

    // Calling The Tuple Structs values
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // Note that black and origin are different values types
    // Because they are instances of different tuple struct
    // Each struct we define is its own type , even though the fields within
    // the struct have the same types .
}
