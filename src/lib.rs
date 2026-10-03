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
//
// Modules aren't useful only for organazing our code . They also define Rust's privacy boundary :
// The line that encapsulates the implementation details make an item like a function or struct
// private , we put it in a module .
// The way privacy works in Rust is that all items (functions , methods , structs , enums and
// modules , and constants) are private by default . items in parent module can't use the private
// items inside child modules , but items in child modules can use the items in their ancestor
// module .
// The reason is that child modules wrap and hide their implementation details , but the child
// modules can see the context in whoich they're defined .
//
//
// Rust chose to have the modules system function this way so that hiding inner implementation
// details is the default . that way , you know which parts of the inner code you can change without
// breaking outer code . But you can expose inner parts of child modules code to outer ancestor by
// using the pub keyword to make an item public .
