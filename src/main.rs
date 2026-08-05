// USING STRUCTS TO STRUCTURE RELATED DATA
// Defining and Instantiating Structs
// User Struct Definition
// Structs are like Objects in Languages like Javascript
// Using Tuple Structs Without Named Fields to create Different Types
// The Blueprint
// Unit-like Structs without Any Fields
//
// Creating q Program using Structs
// Main function
fn main() {
    // Rectangle
    let rect1 = (30, 50);
    // Printing the Area
    println!("The Area of the rectangle is {} square pixels", area(rect1))
}

// Calculating the area of a rectangle specified by separate width and height variables
// Refactoring with Structs
// We add structs to add meaning by labeling the data
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
