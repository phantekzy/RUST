// Paths for Referring to an Item in the Module Tree
//
// To show Rust where to find an item in module tree , we use a path in the same way we use a path
// when navigating a filesystem .
// If we want to call  a function , we need to know its path : A path can take two forms :
// An absolute path starts fron a crate root by using a crate name or a literal crate .
// A relative path starts from the current module and uses self , super , or an identifier in the
// current module .
//
// Both absolute and relative paths are followwed by one or more identifiers separated by double
// colons " :: "
//
// let's Return to our last exemple , How do we call the add_to_waitlist function ? , we simplified
// our ode a bit by removing sone of the modules and functions .
// I will show two ways to call the add_to_waitlist function from a new function eat_at_restaurant
// difned in the crate root .
// The eat_at_restaurant function is part of our library crate's public API , so we mark it with the
// pub keyword , In Exposing Paths with the pub keyword , we will go into more detail about pub.
//
// Example :
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // absolute Path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist()
}
// Calling the add_to_waitlist function using absolute and relative paths
