// Chapter 10 : Generic types ,trauts , and lifetimes
// Removing Dublication by Extracting a function
// Befpre diving into generics syntax, let's first look at how to remove duplication that doesn't
// involve generic types by extracting a function. Then we'll apply this techniaue to extract a
// generic function in the same way that we recognize dublicated code to extract into a function,
// we'll start to recognize duplicated code that can use generics.
//
// Main function
fn main() {
    // Code to find the largest number in a list of numbers
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];
    println!("{}", largest);
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("the largest number is {}", largest)
    // This code stores a list of integers in the variable number_list and places the first
    // number in the list in a variable named largest .
}
