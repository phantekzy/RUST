//THE STACK AND THE HEAP
// OWNERSHIP
// Return values and scope
fn main() {}

fn gives_ownership() -> String {
    // gives_ownership will move
    // its return value into the
    // function that calls it
    let some_string = String::from("String");
    // some_string comes into scope
    some_string
    // some_string is returned and moves out to the
    // calling function
}

// takes_and_gives_back will take a string and return one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling value
}
