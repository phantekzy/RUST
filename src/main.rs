// Chapter 10 : Generic types ,trauts , and lifetimes
// Generic Data Types
// In Struct Definitions :
// The syntax for using generics in struct definitions is similar to that used in function
// definitions. First, we declare the name of the type parameter inside angle brackets just  after
// the name of the struct. then we can use the generic type in the struct definition where we would
// otherwise specify concrete data types.
// Note that because we've use only one generic type to define Point<T>, this definition says that
// the Point<T> struct is generic over some type T, and the fields x and y are both that same type,
// whatever that type may be . If we create an instance of a Point<T> that has values of different
// types, as in this example our code won't compile .
//
//
// In this example, we can change the definition of Point to be generic over types T and U where x is of type T and y is of type U :
struct Point<T, U> {
    // A Point<T,U> generic over two types so that x and y can be values of
    // different types.
    x: T,
    y: U,
}
// Now all the instances of Point shown are alloed! You can use as many generic type parameters in a
// definition as you want,but using more than a few makes your code hard to read. When we need lots
// of generic types in our code, it could indicate that our code need restructuring into smaller
// pieces.
// Main function
fn main() {
    let wont_work = Point { x: 4, y: 6.9 };
    // The fields x and y must be the same type because both have the same generic data type T.
    // In this example, when we assign the integer value 5 to x, we let the compiler know that the
    // generic type T  will be an integer for this instance of Point<T>. Then when we specify 4.0
    // for y, which we've defined to have the same type as x , we'll get a type mistmatch error.
    // ^^^ expected integer, found floating-point number
    // To defint a Point struct where x and y are both generics but could have different types, we
}
