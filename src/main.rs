// Method
// Associated Functions
// Associated functions are often used for constructors that will return a new instance of the struct
// Multiple impl Blocks

// Rectangle Struct
// Each struct is allowed to have multiple impl blocks
struct Rectangle {
    width: u32,
    height: u32,
}
// We could also write the methods and functions in different impl
// First impl for area Method
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
// Second impl for can_hold Method
impl Rectangle {
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

    // To call an Associated function we use the " :: " syntax
    let sq = Rectangle::square(3);
}
