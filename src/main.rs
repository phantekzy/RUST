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

// Main function
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
}
