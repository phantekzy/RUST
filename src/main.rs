// Method Syntax
// Methods are similar to functions : they're declared with the fn keyword and their name
// they can have parameters and return a value
// Methods always have Self as first parameter
// Defining Methods
// Where is the -> Operator
// In languages like C and C++ you have to manualy switch operators depending on whether you hold
// the object directly or hold a pointer to it
// Direct object : object.method();
// Pointer to object : object_ptr->method(); (which is syntax sugar for (*object_ptr).method();)
//
// Rectangle Struct
struct Rectangle {
    width: u32,
    height: u32,
}
// Implementation block
impl Rectangle {
    // Defining an area method on the Rectangle struct
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
