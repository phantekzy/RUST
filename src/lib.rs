// Let's start here , and move the front_of_house module to its own file src/front_of_house.rs
// By changing the crate root file so it contains the code .
// In this case , the crate root file is src/lib.rs , but this procedure also works with binary
// crates whose crate root file is src/main.rs
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
