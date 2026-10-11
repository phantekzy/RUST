// COMMON COLLECTIONS
// Dropping a Vector Drops its Elements
// Like any other struct , a vector is freed when it goes out of scope
fn main() {
    let v = vec![1, 2, 3, 4];
    // Do stuff with v
} // => v goes out of scope and is freed here 
// When the vector gets dropped , all of its contents are also dropped , meaning those integers it
// holds will be cleaned up . This may seem like a straightforward point but can get a bit more
// complicated when we stard to introduce references to the element of the vector .
