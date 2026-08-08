// USING STRUCTS TO STRUCTURE RELATED DATA
// Adding usefull functionalities with Derived Traits
// Let's try to print the instance of rectangle while debugging

// Rectangle Struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// help: the trait `std::fmt::Display` is not implemented for `Rectangle`
// note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
// Lets try this formats
// Main function
fn main() {
    // Instance of Rectangle Struct
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    // Adding the annotation to derevie the Debug trait and printing the Rectangle instance using
    // debug formating
    println!("rect1 is {:?}", rect1);
}
