// STRUCTS Defining and Instantiating Structs PS : To define a struct we enter the keyword Struct and name the entire struct
// CREATING INSTANCES FROM OTHER INSTANCES USING THE STRUCT UPDATE SYNTAX
struct User {
    username: String,   // String
    email: String,      // We call the data inside
    sign_in_count: u64, // The curly brackets "fields"
    active: bool,       // Boolean
}

// How to use the Struct
// MAIN FUNCTION
fn main() {
    // Making the Struct mutable
    // You cannot change the values if the entire instance is not mutable
    let mut user1 = User {
        email: String::from("xxx@mail.com"),
        username: String::from("xxx69"),
        active: true,
        sign_in_count: 1,
    };
    user1.username = String::from("Phantekzy");
    // STRUCT SYNTAX UPDATE
    let user2 = User {
        email: String::from("another@example"),
        username: String::from("anotherusername67"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
}

// Creating a build_user function that returns a User instance
// with the given email and password
// Filed init shorthand
fn build_user(email: String, username: String) -> User {
    User {
        username, // FIELD INIT SHORTHAND
        email,    // FIELD INIT SHORTHAND
        sign_in_count: 1,
        active: true,
    }
}
