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
    let width1 = 30;
    let height1 = 50;
    // Printing the Area
    println!(
        "The Area of the rectangle is {} square pixels",
        area(width1, height1)
    )
}

// Calculating the area of a rectangle specified by separate width and height variables
fn area(width: u32, height: u32) -> u32 {
    width * height
}
