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
impl Rectangle {
    // Added _ so the compiler shut the hell up
    fn _area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
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
    // Printing the Results
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
