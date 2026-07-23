// A Program, Using Struct
// Refactoring with Structs adding more meaning this time

// Rectangle Struct
struct Recrangle {
    width: u32,
    height: u32,
}

// Main Function
fn main() {
    let rec1 = Recrangle {
        width: 30,
        height: 50,
    };
    println!(
        "
    The area of the rectangle is {} square pixels",
        area(&rec1)
    )
}

// Area function using the rectangle reference from the struct
fn area(rectangle: &Recrangle) -> u32 {
    rectangle.height * rectangle.width
}
