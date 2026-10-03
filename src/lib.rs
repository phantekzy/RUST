// Starting Relative Paths with super
// We can als o construct relative paths that begin in the parent module by using super at the start
// of the path . This is like starting a filesystem path with the .. syntax.
//
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // absolute Path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist()
}

// Serveing orders function
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {}
    fn cook_order() {}
}
