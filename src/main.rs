// STRUCTS
// Defining and Instantiating Structs

// PS :
// To define a struct we enter the keyword Struct
// and name the entire struct

struct User {
    username: String,   //
    email: String,      // We call the data inside
    sign_in_count: u64, // the curly brackets "fields"
    active: bool,       //
}
// How to use the Struct
// MAIN FUNCTION
fn main() {
    // Making the Struct mutable
    let mut user1 = User {
        email: String::from("xxx@mail.com"),
        username: String::from("xxx69"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@email.com");
}
