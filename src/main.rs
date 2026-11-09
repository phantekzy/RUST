// Chapter 10 : Generic types ,trauts , and lifetimes
// Generic Data Types
// In Method Definitions :
// We can implement methods on structs and enums and use generic types in their definitions,
// too.This example shows the Point<T> struct we defined in one of the past examples with a method
// named x implemented on it
struct Point<T> {
    x: T,
    y: T,
}
// We could,for example,implement methods only on Point<f32> instances rahter than Point<T>
// instances with any generic type. In this example, we use the concrete type f32, meaning we don't
// declare any types after impl.
// Main function
fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x())
}
