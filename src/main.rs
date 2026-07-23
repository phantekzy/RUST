// Refactoring with Structs adding more meaning this time

// Rectangle Struct

struct Recrangle {
    width: u32,
    height: u32,
}

// Main Function
fn main() {
    // Adding Useful Functionality with Derived Traits
    let rec1 = Recrangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {}", rec1);
}
