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
// The largest_i32 function is the one we extracted in the past that finds the largest i32 in a
// slice. The largest_char function finds the largest char in a slice too.
// The function bodies have the same code , so let's eleminate the duplication by interducing a
// Generic type parameter in a single function.
//
// To parameterize the types in the new function wi'll define, we need to name the type parameter,
// just as we do for the value parameters to a function. We can use any identifier as a type
// parameter name. But we'll use T because, by convention, parameters names in Rust are short,
// oftern just a letter, and Rust's type naming convention is CamelCase. Short for "type", T is the
// default choice of most Rust proggrammers.
// When we use a parameter in the body of the function, we have to declare the parameter name in the
// signature so the compiler kknows what that name means. Similarly, when we use a type parameter
// name in a function signature , we have to declare the type parameter name before we use it. To
// define the generic largest function ,we place type name declarations inside angle brackeets, <>,
// between the name of the function and the parameter list :
// We read this definition as: the function largest is generic over some type T. This function has
// one parameter named list, which is a slice of values of type T. The largest function will return
// a value of the same type T.
// This example shows t he combined largest function definition using the generic data type in its
// signature. The example also shows how we can call the function with either a slice of i32 value
// or char values. Note that this code won't compile yet , we will fix it later

fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Main function
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number in the list is {}", result);
    let char_list = vec!["y", "m", "a", "i"];
    let result = largest(&char_list);
    println!("The largest char in the list is{}", result);
    // A definition of the largest function that uses generic type parameters but doesn't compile !
    // If we compile this code right now we'll get this error :
    //    Compiling rusting v0.1.0 (/home/phantekzy/RUST)
    //error[E0369]: binary operation `>` cannot be applied to type `T`
    // --> src/main.rs:51:17
    // if item > largest {
    //            ---- ^ ------- T
    //help: consider restricting type parameter `T` with trait `PartialOrd`
    // 47 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
    //             ++++++++++++++++++++++

    //For more information about this error, try `rustc --explain E0369`.
    //error: could not compile `rusting` (bin "rusting") due to 1 previous error
}
