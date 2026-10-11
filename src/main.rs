// COMMON COLLECTIONS
// Iterating over the Values in a Vector
// If we want to access each element in a vector in turn , we can iterate through all of the
// elements rather than use indices to access one at a time .
// This example shows how to use a for loop to get immutable references to each element in a vector
// of i32 values and print them .
// Main function
fn main() {
    let mut v = vec![100, 32, 57];
    // Printing each element in a vector by iterating over the elements using a for loop
    // We can also iterate over mutable references to each element in a mutable vector in order to make
    // change to all the elements
    for i in &mut v {
        // Iterating over mutable references to elements in a vector
        *i += 50; // Dereferensing 
    }
    // To change the value that the mutable reference refers to, we have to use the dereference
    // operator (*) to get to the value in i before we can use the += operator .
}
