//THE STACK AND THE HEAP
// OWNERSHIP
// OWNERSHIP AND FUNCTIONS
fn main() {
    let s = String::from("FUNCTIONS"); // s comes into scope 
    takes_ownership(s); // s value moves into the function 
    // s is not valid here cause the ownershop has moved
    let x = 5;
    makes_copy(x);
} // Here x goes out of scope , then s

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string)
} // Here some_string goes out of scope and 'drop' function is called 
// Memory is freed

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer)
} // Here some_integer goes out of scope . Nothing special happens
