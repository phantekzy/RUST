// Exposing Paths with the pub keyword
//
// Let's return to the error we had that told us the hosting module is private .
// We want the eat_at_restaurant function in the parent module to have access to the add_to_waitlist
// function in the parent module , so we mark the hosting module with the pub keyword .
//
//
// Example :
mod front_of_house {
    // Declaring the hosting module as pub to use it from eat_at_restaurant
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // absolute Path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist()
}
// What happend ? Adding the pub keyword in front of mod hosting makes the module public .
// With this change , if we can access front_of_house , we can access hosting . but the contents of
// hosting is still private ; making the module public doen;t make its contents public .
