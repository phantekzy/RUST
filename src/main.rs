// The Glob Operator
// If we want to bring all public items defined in a path into scope , we can specify that path
// followed by * , the glob oerator :
use std::collections::*;
// This use statement brings all public items defined in std::collections into the current scope .
// Be carefull when using the glob operator ! Gloc can make it harder to tell what names are in
// scope and where a name is used in your program was defined .
// The glob operator is often used when testing to bring everything under test into the tests module
// The glob operator is also sometimes used as part of the prelude pattern .
// Main function
fn main() {}
