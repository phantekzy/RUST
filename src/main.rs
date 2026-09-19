// OPTION TYPE
// Option<T> Enum
// Main function
fn main() {
    let _absent_number: Option<i32> = None;
    let _some_number = Some(5);
    let _some_string = Some("a String");

    let x: i8 = 5;
    let y: Option<i8> = Some(5); // In order to have a value that can possibly be null,
    // we must explicitly opt in by making
    // the type of that value Option<T>

    // We cant add because they are different types
    // We have to convert an Option<T>  to a T before performing T operations with it.
    // This helps catch one of the most common issues with null :
    // assuming  that something isnt't null when it actually is.
    let sum = x + y;
}
