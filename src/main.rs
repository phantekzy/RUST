// Method Syntax
// Methods are similar to functions : they're declared with the fn keyword and their name
// they can have parameters and return a value
// Methods always have Self as first parameter
// Defining Methods

// Rectangle Struct
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    // Defining a method
    // It has self in parameters
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

// Main function
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    print!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    )
}
