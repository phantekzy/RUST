// Chapter 10 : Generic types ,trauts , and lifetimes
// Generic Data Types
// In Method Definitions :
// We can implement methods on structs and enums and use generic types in their definitions,
// too.This example shows the Point<T> struct we defined in one of the past examples with a method
// named x implemented on it
//
//
// Here, we've defined a method named x on Point<T> that returns a  reference to the data in the
// field x.
// Note that we have to declare T just aftwe impl so we can use it to specify that we're
// implementing methods on the type Point<T>. By declaring T as a generic type after impl, Rust
// can identify that the type in the angle brackets in Point is a generic type rather than a
// concrete type.
// We could,for example,implement methods only on Point<f32> instances rahter than Point<T>
// instances with any generic type. In this example, we use the concrete type f32, meaning we don't
// declare any types after impl.
// This code means the type Point<f32> will have a method named distance_from_origin and other
// instaances of Point<T> where T is not of type f32 will not have this method defined.
// The method measures how far our point is from the point at coordinates (0.0 , 0.0) amd uses
// mathematical operations that are available only for floating point types.
// Generic type parameter in a struct definition aren't always the same as those you use in that
// struct's method signatures. For example the next example defines the method mixup on the
// Point<T,U> struct from our past examples. The method  takes another Point as a parameter,
// which might have different types thant the self Point we're calling mixup on. The method
// creates a new Point instance with the x value fron the self Point(of type T) and the y value
// from the passed-in Point (of type W).

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point<T, U> {
    // A method that uses different generic types than its struct's definition
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// Main function
fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("{:?}", p3);
}
