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

// Fixing the bug
impl Recrangle {
    fn area(self: &Self) -> u32 {
        self.width * self.height
    }
}

// Main Function
// Defining Methods
fn main() {
    let rec1 = Recrangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rec1.area()
    )
}
