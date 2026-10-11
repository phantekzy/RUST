// COMMON COLLECTIONS
// Using an Enum to store Multiple Types :
// At the beginning of this chapter , we said that vectors can only store values that aret he same
// type. This can be inconvenient; There are definitely use cases for needing to store a list of
// items of different types . Fortunately , the variants of an enum are defined under the same enum
// type , so when we need to store elements of a different type in a vector , we can define and use
// an enum !
// For example , say we want to get values from a row in a spreadsheet in which some of the columns
// in the row contain integers , some floating point numbers and some strings .
// We can define an enum whose variants will hold the diferent values types , and then all the enum
// variants will be considered the same type : that of the enum .
// Then we can create a vector that holds that enums and so
// Main function
fn main() {}
