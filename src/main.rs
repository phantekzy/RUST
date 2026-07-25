// Methods with more Parameters
// Real Example
struct Rectangle {
    height: u32,
    width: u32,
}
impl Rectangle {
    fn _area(&self) -> u32 {
        self.width * self.height
    }
    // Implementing the method on Rectangle that takes
    // another Rectangle instanece as a parameter
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
    // Using the as-yet-unwritten can hold method
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
