// Associated functions
// We are allowed to define functions within impl blocks that don't
// take "self" as parameter
// we call these function "Associated functions" because they are
// associated with the struct

// Associated function are often used for constructors that will
// return a new instance of the struct

// Multiple impl Block
// Each struct is allowed to have multiple impl blocks
// Rectangle Struct
// PS :
// There is no reason to separate these methods but it is a valid syntax
struct Rectangle {
    height: u32,
    width: u32,
}

// First Impl
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // Associated Function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            height: size,
            width: size,
        }
    }
}
// Second Impl
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Main Function
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
    // Method
    println!("The area of the first Rectangle is {}", rect1.area());
    // Using the as-yet-unwritten can hold method
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
