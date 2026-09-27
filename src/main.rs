// MANAGIN GROWING PROJECTS WITH PACKAGES , CRATES AND MODULES
//
//
// As we write large programs , organizing our code will be important because keeping
// track of our entire program in our head will become impossible .
// By regrouping related functionality and separating code with distinct features , we'll clarify
// where to find code that implements a particular feature and where to go to change how
// a feature works .
//
// The programms we've weitten so far have been in one module in one file .
// As a project grows , we can organize code by splitting it into multiple modules and then
// multiply files .
// A package can contain multiple binary crates and optionally one library crate .
// As a package grows , we can extract parts into separate crates that become external dependencies
//
// In addition to grouping functionality , encapsulating imple;entation details lets us reuse code
// at a higher level : once we've implemented an operation,other code can call that code via the
// code's public interface without knowing how the implementation works .
// The way we write the code defines whoch parts are public for other code to use and whch parts
// are private implementation details that we reserve the right to change . This is another way to
// limit the amount of detail we have to keep in our head .
// A related conceptt is scope : the nested context in which code is written has a set of names that
// are defined as "in scope".

fn main() {}
