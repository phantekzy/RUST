// COMMON COLLECTIONS
// Iterating over the Values in a Vector
// If we want to access each element in a vector in turn , we can iterate through all of the
// elements rather than use indices to access one at a time .
// This example shows how to use a for loop to get immutable references to each element in a vector
// of i32 values and print them .
// Main function
fn main() {
    let v = vec![100, 32, 57];
    // Printing each element in a vector by iterating over the elements using a for loop
    for i in &v {
        println!("{}", i)
    }
}
