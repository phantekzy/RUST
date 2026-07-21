// A Program, Using Struct

// Main Function
fn main() {
    let width1 = 30;
    let height1 = 50;
    // Calculating the Area using the function bellow
    println!(
        "The Area of the rectangle is {} square pixels.",
        area(width1, height1)
    )
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
