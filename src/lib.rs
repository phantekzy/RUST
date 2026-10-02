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
// The first time we call the add_to_waitlist function in eat_at_restaurant, we use an absolute path
// The add_to_waitlist function is defined in the same crate as eat_at_restaurant, whoch means we
// can use the create keywordto start an absolute path .
//
// After crate , we include each of the succesive modules until we make our way to add_to_waitlist .
// We can imagine a filesystem with the same structure , and we'd specify the path
// /front_of_house/hosting/add_to_waitlist to run the add_to_waitlist program ; using the crate name
// to start from the crate root is like using  '/' to start from the filesystem root in our shell .
//
// The second time we call add_to_waitlist in eat_at_restaurant , we use a relative path . the path
// starts with front_of_house , the name of the module defined at the same level of the module tree
// as eat_at_restaurant .
// Here the filesystem equivalent would be using the path front_of_house/hosting/add_to_waitlist.
// Starting with a name means that the path is relative .
//
// Choosing whether to use a relative or absolute path is a decision we will make based on your
// project . The decision should depend on wheter we are more likely to move item definition code
// separately from or together with the code that uses the item .
