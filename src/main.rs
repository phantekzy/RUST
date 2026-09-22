// Matching with Option<T>
// In the previous section , we wanted to get the inner T value out of t he Some case when using
// Option<T>; we can also handle Option<T> using match as we did with the Coin enum ! Instead of
// comparing coins , we will compare the variants of Option<T> , but the way that the match
// expression works remains the same .
//
// Let's say we want tot write a function that takes an Option<i32> and , if there's a value inside
// , adds 1 to that value , If there isn't a value inside , the function should return the None
// value and not attempt to perform any operations .
// Main function
fn main() {}
