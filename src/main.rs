// Rust Automatic Referencing and Dereferencing
// Where is the -> Operator ?
// In C and C++ , two different operators are used for calling methods : we use the . operators
// if we are calling a method on an Object directly
// and we use the -> operator if we are calling the method on a pointer to the object and
// need to dereference the pointer first

// In other words:
// if object is a pointer : object -> something() is similar to : (*object).something()

// So this how it works :
// when we call a method with object.something() :
// Rust will automaticly add in & , &mut or *
// so Object matches the signature of the method

// Real Example
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// Main Function
fn main() {}
