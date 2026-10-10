// Let's start here , and move the front_of_house module to its own file src/front_of_house.rs
// By changing the crate root file so it contains the code .
// In this case , the crate root file is src/lib.rs , but this procedure also works with binary
// crates whose crate root file is src/main.rs

mod front_of_house;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
