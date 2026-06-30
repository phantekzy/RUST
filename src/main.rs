//THE STACK AND THE HEAP
// OWNERSHIP
// OWNERSHIP AND FUNCTIONS
fn main() {
    let s = String::from("FUNCTIONS");
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string)
}

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer)
}
