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
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The Area of the rectangle is {} square pixels.",
        area(&rect1)
    )
}

// The function takes the Rectangle Struct as parameter
// It is an immutable borrow because its a read only
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
