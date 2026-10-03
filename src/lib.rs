// Paths for Referring to an Item in the Module Tree
//
//
// Example :
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // absolute Path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist()
}

// Let's try to compile and find out why it won't compile yet !
// Compiler errors from building the code :
// The error messages say that the module hosting is private . in other words, we have the correct
// paths for the hosting module and the add_to_waitlist function , but Rust won't let us use them
// because it does't have access to private sections .
