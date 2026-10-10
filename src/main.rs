// Using External Packages
// In Chapter 2 , we programmed a guessing game project that used an external package called rand to
// get random numbers . to use rand in our project , we added that line to Cargo.toml
// rand = "0.5.5"
// Adding rand as a dependency in Cargo.toml tells Cargo to download the rand package and any
// dependencies from crates.io and make rand available to our project .
// Then , to bring rand definitions into the scope of our Packages , we added a use line starting
// with the name of the package , rand , and listed the items we wanted to bring into scope .

use rand::Rng;

// Main function
fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
}

// Note that the standard library (std) is also a crate that's external to our package .
// Because the standard library is shipped with the Rust langauge , we do not need to change
// cargo.toml to include std.
// But we do need to refer to it with use to bring items from there into our package's scope.
// For example with Hashmap we would use this line :
// use std::collections::Hashmap ;
// This is an absolute path starting with std , the name of the standard library crate .
