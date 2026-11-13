// Chapter 10 : Generic types ,trauts , and lifetimes
// Generic Data Types
// In Method Definitions :
// We can implement methods on structs and enums and use generic types in their definitions,
// too.This example shows the Point<T> struct we defined in one of the past examples with a method
// named x implemented on it
//
//
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// Here, we've defined a method named x on Point<T> that returns a  reference to the data in the
// field x.
// Note that we have to declare T just aftwe impl so we can use it to specify that we're
// implementing methods on the type Point<T>. By declaring T as a generic type after impl, Rust
// can identify that the type in the angle brackets in Point is a generic type rather than a
// concrete type.
// We could,for example,implement methods only on Point<f32> instances rahter than Point<T>
// instances with any generic type. In this example, we use the concrete type f32, meaning we don't
// declare any types after impl.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    } // An impl block that only applies to a struct with a particular concrete type for the generic
    // type parameter T.
    // This code means the type Point<f32> will have a method named distance_from_origin and other
    // instaances of Point<T> where T is not of type f32 will not have this method defined.
    // The method measures how far our point is from the point at coordinates (0.0 , 0.0) amd uses
    // mathematical operations that are available only for floating point types.
    // Generic type parameter in a struct definition aren't always the same as those you use in that
    // struct's method signatures. For example the next example defines the method mixup on the
    // Point<T,U> struct from our past examples.
}

// Main function
fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}
