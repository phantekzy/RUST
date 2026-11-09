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
// Implementing a method named x on the Point<T> struct that will return a reference to the x field
// of type T
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// Here, we've defined a method named x on Point<T> that returns a reference to the data in the
// field x.
// Note that we have to declare T just after impl so we can use it to specify that we're
// Implementing methods on the type Point<T>. By declaring T as a generic type after impl, Rust can
// identify that the type in the angle brackets in Point is a generic type rather than a concrete
// type.
// We could,for example,implement methods only on Point<f32> instances rahter than Point<T>
// instances with any generic type. In this example, we use the concrete type f32, meaning we don't
// declare any types after impl.
// Main function
fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x())
}
