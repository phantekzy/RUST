// Associated functions
// We are allowed to define functions within impl blocks that don't
// take "self" as parameter
// we call these function "Associated functions" because they are
// associated with the struct

// Associated function are often used for constructors that will
// return a new instance of the struct
struct Rectangle {
    height: u32,
    width: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // Implementing the method on Rectangle that takes
    // another Rectangle instanece as a parameter
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // Associated Function
    // To call this function we use the "::" syntax with the struct name
    fn square(size: u32) -> Rectangle {
        Rectangle {
            height: size,
            width: size,
        }
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
