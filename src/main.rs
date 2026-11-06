// Chapter 10 : Generic types ,trauts , and lifetimes
// Generic Data Types
// We can use generics to create definitions for items like functions signatures or structs, which
// we can then use with many different concrete data types.
// let's look at how to define functions, structs, enums, and methods using generics.
//
// In function definitions :
// When defining a function that uses generics, we place the generics in the signature of the
// function where we would usually specify the data types of the parameters and return value .
// Doing so makes our code more flexible and provides more functionality to callers of our function
// while preventing code duplication.
// Continuing with our largest function this example shows two functions that both find the largest
// value in a slice.

// Two functions that differ only in their names and the type their signatures
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Main function
fn main() {}
