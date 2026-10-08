// Creating Idiomatic use Path
// If you have wondred why we specified use crate::front_of_house::hosting and thenb called
// hosting::add_to_waitlist in eat_at_restaurant rather than sepcifying the use path all the way out
// to the add_to_waitlist function to achieve the same result .

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
// Bringing the add_to_waitlist function into scope with use , which is unidiomatic
use crate::front_of_house::hosting::add_to_waitlist;
pub fn eat_at_restaurant() {
    add_to_waitlist();
}
