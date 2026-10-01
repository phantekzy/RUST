// Let's define some modules and function signatures in this library crate !
//
// Library section
// Frong of the house
mod front_of_house {
    // Hosting Section
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
