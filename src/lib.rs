// Let's start here , and move the front_of_house module to its own file src/front_of_house.rs
// By changing the crate root file so it contains the code .
// In this case , the crate root file is src/lib.rs , but this procedure also works with binary
// crates whose crate root file is src/main.rs

mod front_of_house;
// Using a semicolon after mod front_of_house rather than using a block tells Rust to load the
// contents of the module from another file with the same name as the module .
pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
