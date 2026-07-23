// Method Syntax
// Mehtods are similar than Functions
// Methods are defined within the context of a Struct
// and their first parameter is Always 'Self'

// Rectangle Struct
#[derive(Debug)] // Derive Annotation
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

    println!("rect1 is {:?}", rec1);
    println!("rect1 is {:#?}", rec1);
    println!("The Area calculated is {}", area(&rec1))
}

// Area calculation function
fn area(rectangle: &Recrangle) -> u32 {
    rectangle.width * rectangle.height
}
