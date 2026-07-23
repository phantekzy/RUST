// Refactoring with Structs adding more meaning this time
// FORMATING SPECIFIERS
//
// {:?} => Debug formatter
// {:#?} => Pretty print Debug formatter
//

// Rectangle Struct
#[derive(Debug)]
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

    // The println macro can do many kind of formating
    // By Default the curly brackets {} tells the println
    // to use formating known as Display: output
    // intended for direct end user consumption
    // Debug Formatter
    println!("rect1 is {:?}", rec1);
    // Pretty Print Debug Formatter
    println!("rect1 is {:#?}", rec1);
    println!("The Area calculated is {}", area(&rec1))
}

// Area calculation function
fn area(rectangle: &Recrangle) -> u32 {
    rectangle.width * rectangle.height
}
