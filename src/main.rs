struct User {
    username: String,   // String
    email: String,      // We call the data inside
    sign_in_count: u64, // The curly brackets "fields"
    active: bool,       // Boolean
}

// MAIN FUNCTION
fn main() {
    // TUPLE STRUCTS
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // ASSIGNING THE VARIABLES
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
