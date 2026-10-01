// Let's define some modules and function signatures in this library crate !
//
// Library section
// Frong of the house module containing other modules that then contain functions
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    // Serving Section
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

// We define a module by starting with the mod keyword and then specify the name of the module (in
// this case , front_of_house ) and place a curly brackets around the body of the module
// Inside modules , we can have other modules , as in this case with the modules 'hosting' and
// 'serving' .
// Modules can also hold definitions for other items , such as structs , enums , constants , traits.
// PS : Definition is the logic
