// Method
// Methods with more Parameters
// Let us implement a second method on the Rectangle struct
// This time , i want an instance of Rectangle to take another instance of Rectangle
// and return true if the second Rectangle can fit completely within self
// otherwise it should return false

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
