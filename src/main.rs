// Using External Packages
// In Chapter 2 , we programmed a guessing game project that used an external package called rand to
// get random numbers . to use rand in our project , we added that line to Cargo.toml
// rand = "0.5.5"
// Adding rand as a dependency in Cargo.toml tells Cargo to download the rand package and any
// dependencies from crates.io and make rand available to our project .
// Then , to bring rand definitions into the scope of our Packages , we added a use line starting
// with the name of the package , rand , and listed the items we wanted to bring into scope .
// Main function
fn main() {}
