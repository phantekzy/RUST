// Re-exporting names with pub use
// When we bring a name into scope with the use keyword , the name available in the new scope is
// private .
// To enable the code that calls our code to refer that name as if it had been defined in that
// code's scope , we can combine pub and use . This technique is called Re-exporting because we're
// bringing an item into scope but also making that item available for others to bring into their
// scope .

pub use crate::front_of_house::hosting; // Making a name available for any code to use a new scope
// with pub use .
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// By using pub use , external code can now call the add_to_waitlist function using
// hosting::add_to_waitlist function using hosting::add_to_waitlist
//
// Testing the public exporting
// A deeplu hidden internal module
mod database {
    pub mod postgres {
        pub fn connect() {
            println!("Connected to postgreSQL");
        }
    }
}
