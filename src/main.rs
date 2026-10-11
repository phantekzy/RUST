// COMMON COLLECTIONS
// Dropping a Vector Drops its Elements
// Like any other struct , a vector is freed when it goes out of scope
fn main() {
    let v = vec![1, 2, 3, 4];
    // Do stuff with v
} // => v goes out of scope and is freed here 
