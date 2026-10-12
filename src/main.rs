// COMMON COLLECTIONS
// Using an Enum to store Multiple Types :
// At the beginning of this chapter , we said that vectors can only store values that aret he same
// type. This can be inconvenient; There are definitely use cases for needing to store a list of
// items of different types . Fortunately , the variants of an enum are defined under the same enum
// type , so when we need to store elements of a different type in a vector , we can define and use
// an enum !
// For example , say we want to get values from a row in a spreadsheet in which some of the columns
// in the row contain integers , some floating point numbers and some strings .
// We can define an enum whose variants will hold the diferent values types , and then all the enum
// variants will be considered the same type : that of the enum .
// Then we can create a vector that holds that enums and so
// #[derive(Debug)]
enum SpreadsheetCell {
    // Defining an enum to store values of different types in one vector
    Int(i32),
    Float(f64),
    Text(String),
}
// Main function
fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("phantekzy")),
        SpreadsheetCell::Float(10.12),
    ];
}
// Rust needs to know what types will be in the vector at compile time so it knows exactly how much
// memory on the heap will be needed to store each element.
// A secondary advantage is that we can be explicit about what types are allowed in this vector .
// If Rust allowd a vector to hold any type , there would be a change that one or more of the types
// would cause errors with the operations performed on the elements of the vector .
// Using an enum plus a match expression means that Rust will ensure at compile time that every
// possible case is handled .
