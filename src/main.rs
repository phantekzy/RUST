// Creating Idiomatic use Path
// If you have wondred why we specified use crate::front_of_house::hosting and thenb called
// hosting::add_to_waitlist in eat_at_restaurant rather than sepcifying the use path all the way out
// to the add_to_waitlist function to achieve the same result .

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
// Bringing fmt and io to scope
// A we can see , using parent modules distinguishes the two Result types
// If instead we specified use std:fmt::Result and use std::io:Result , we'd have two Result types
// in the same scope and Rust wouldn't know which one we meant when we used Result
//
use std::{fmt, io};
// Bringing two types with the same name into the same scope requires using their parent modules .
// Bringin HashMap into scope in an idiomatic way
use std::collections::HashMap;
// There is no strong reason behind this idiom : it's just a convention that has emerged , and folks
// have gotten used to reading and writing Rust code this way
//
// The exception in this idiom is if we're bringing two items with the same name into scope with use
// statement , because Rust doesn't allow that .
//
// In this example we will show how to bring two Results types into scope that have the same name
// but different parent module and how to refer to them .
//

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}

// Bringing the add_to_waitlist function into scope with use , which is unidiomatic
use crate::front_of_house::hosting::add_to_waitlist;
pub fn eat_at_restaurant() {
    add_to_waitlist();
}
// Although both listings accomplish the same task , the first is the adiomatic way to bring a
// function inso scope with use .
// Bringing the function's parent module into scope with use so we have to specify the parent module
// when calling the function makes it clear that the function isn't locally defined while still
// minimizing repetition of the full path .
//
// On the other hand , when Bringing in structs , enums , and other items wuth use , it's idiomatic
// to specify the full path .
//
// Main function
fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
