// Chapter 10 : Generic types ,trauts , and lifetimes
// Removing Dublication by Extracting a function
// Befpre diving into generics syntax, let's first look at how to remove duplication that doesn't
// involve generic types by extracting a function. Then we'll apply this techniaue to extract a
// generic function in the same way that we recognize dublicated code to extract into a function,
// we'll start to recognize duplicated code that can use generics.
//
// Main function
fn main() {
    // Abstracted code to find the largest number in two lists
    let number_list = vec![34, 50, 25, 100, 65];
    let results = largest(&number_list);
    println!("The largest number is {}", results);
}
//
// In this example , we extracted the code that finds the largest number into a function named
// largest. Unlike the code in the first example , which can find the largest number in only one
// particular list , this program can find the largest number in two different lists.
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
// The largest fucntion has a parameter called list, which represents any concrete slice of i32
// values that we might pass into the function . As a result , when we call the function, the code
// runs on the specific values that we pass in .
