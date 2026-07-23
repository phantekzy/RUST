// Refactoring with Structs adding more meaning this time

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
    println!("rect1 is {:?}", rec1);
}
