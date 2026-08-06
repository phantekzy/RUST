// USING STRUCTS TO STRUCTURE RELATED DATA
// Defining and Instantiating Structs
// User Struct Definition
// Structs are like Objects in Languages like Javascript
// Using Tuple Structs Without Named Fields to create Different Types
// The Blueprint
// Unit-like Structs without Any Fields
//
// Creating q Program using Structs
// Rectangle Struct
struct Rectangle {
    width: u32,
    height: u32,
}
// Main function
fn main() {}

// Calculating the area of a rectangle specified by separate width and height variables
// Refactoring with Structs
// We add structs to add meaning by labeling the data
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
