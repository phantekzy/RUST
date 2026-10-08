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
// Although both listings accomplish the same task , the first is the adiomatic way to bring a
// function inso scope with use .
// Bringing the function's parent module into scope with use so we have to specify the parent module
// when calling the function makes it clear that the function isn't locally defined while still
// minimizing repetition of the full path .
