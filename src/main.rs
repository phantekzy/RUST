// A Program, Using Struct
// Refactoring with Structs adding more meaning this time
// Main Function

// Rectangle Struct
struct Recrangle {
    width: u32,
    height: u32,
}

fn main() {}

// Area function but with a Tuple in parameters
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
