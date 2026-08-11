// Method
// Associated Functions
// Associated functions are often used for constructors that will return a new instance of the struct

// Rectangle Struct
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    // Added _ so the compiler shut the hell up
    // Method
    // PS : All Methods are Associated function in Rust
    // A Method is simply an Associated function that takes self , &self or &mut self
    // as its first params
    // HOW METHOD CALLING WORKS UNDER THE HOOD
    // rect1.can_hold(&rect2);
    // The Rust compiler automatically translates it into the fully qualified Associated function
    // call syntax using " :: "

    fn _area(&self) -> u32 {
        self.width * self.height
    }
    // Method
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // Associated function
    // There is no self in params so its an Associated function

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
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
