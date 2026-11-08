// Chapter 10 : Generic types ,trauts , and lifetimes
// Generic Data Types
// In Struct Definitions :
// We can also define structs to use a generic type parameter in one or more fields using the <>
// syntax. This example shows how to define a Point<T> Struct to hold x and y coordinate values of
// any type.
//
// A Point<T> struct that holds x and y values of type T
struct Point<T> {
    x: T,
    y: T,
}
// Main function
fn main() {
    let integer = Point { x: 5, y: 6 };
    let float = Point { x: 1.2, y: 2.2 };
}
