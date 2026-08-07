// USING STRUCTS TO STRUCTURE RELATED DATA
// Adding usefull functionalities with Derived Traits
// Let's try to print the instance of rectangle while debugging

// Rectangle Struct
struct Rectangle {
    width: u32,
    height: u32,
}
// Main function
fn main() {
    // Instance of Rectangle Struct
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {}", rect1);
}
