// COMMON COLLECTIONS
// Reading Elements of Vectors
// Now that we know how to create , update , and destroy vectors , knowing how to read their
// contents is a good next step .
// There are two ways to reference a value stored in a vector .

// Note two details here , First , we use the index value of 2 to get the third element : vectors
// are indexed by number , starting at zero . Second , the two ways to get the third element are by
// using & and [] , which gives us a reference , or by using the get method with the index passed
// as an argument , which gives us an Option<&T> .
// Rust has two ways to reference an element so we can choose how the program behaves when we try to
// use an index value that the vector doesn't have an element for .
// Let's see what a program will do if it has a vector that holds five elements and then tries to
// access an element at index 100 :

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    // dne refers to does not exist
    let dne = &v[100];
    let dne = v.get(100);
    // When we run thgis code , the first [] method will cause the program to panic because it
    // references a nonexistent element .
}
