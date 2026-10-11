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
    // let dne = &v[100];
    let dne = v.get(100);
    // Printing the results
    println!("{:?}", dne); // Shows None instead of panicking !
    // When we run thgis code , the first [] method will cause the program to panic because it
    // references a nonexistent element .
    // This method is best used when we want our program to crash if there's an attempt to access an
    // element past the end of the vector .
    // When the get method is passed an index is outside the vector , it returns None without
    // panicking . We would use this method if accessing an element beyound the range of the vector
    // happens occasionally under normal circumstances .
    // Our code  will then have logicto handle having either Some(&element) or None.
    // For example , the index could be comming from a person entering a number . if they accidently
    // enter a number that's too large and the program gets a None value , we could tell the user
    // how many items are in the current vector and give them another chance to enter a valid value.
    // That would be more user-friendly than crashing the program due to a type !
    // When the program has a valid reference , the borrow checker enforces the ownership and
    // borrowing rules to ensure this reference and any other references to the contents of the
    // vector remain valid .
}
