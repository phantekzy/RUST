// COMMON COLLECTIONS
// Storing Lists of Values with Vectors
// The first collection type we'll look at is Vec<T> , also known as a vector. Vectors allow you to
// store more than one value in a single data structure that puts all the vales next to each other
// in memory . Vectors can only store values of the same type . They are useful when we have a list
// of items, such as the lines of text in a file or the prices of items in a shopping cart .
//
// Main function
fn main() {
    // Creating a New Vector :
    // To create a new,empty vector , we can call the vec::new function
    let v: Vec<i32> = Vec::new(); // Creating a new , empty vector to hold values of type i32;
    // Note that we added a type annotation here . Because we aren't inserting any values into this
    // vector , Rust doesn't know what kind of elements we intend to store .
    // This is an important point . Vectors are implemented using generics;
    // In more realistic code , Rust can often infer the type of value you want tot store once you
    // insert values , so you rarely need to do this type annotation .
    // It's more common to create a Vec<T> that has initial values , and Rust provides the vec!
    // macro for convenience .
    // The macro will create a new vector that holds the values you give it .
    let y = vec![1, 2, 3];
}
