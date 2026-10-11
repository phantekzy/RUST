// COMMON COLLECTIONS
// Reading Elements of Vectors
// Now that we know how to create , update , and destroy vectors , knowing how to read their
// contents is a good next step .
// There are two ways to reference a value stored in a vector .
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    // Using indexing syntax
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    // using the get method to access an item in a vector
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }
}
// Note two details here , First , we use the index value of 2 to get the third element : vectors
// are indexed by number , starting at zero . Second , the two ways to get the third element are by
// using & and [] , which gives us a reference , or by using the get method with the index passed
// as an argument , which gives us an Option<&T> .
// Rust has two ways to reference an element so we can choose how the program behaves when we try to
// use an index value that the vector doesn't have an element for .
