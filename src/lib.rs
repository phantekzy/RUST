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
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
        // Calling a function using a relative path starting with super
    }
    fn cook_order() {}
}
// The fix_incorrect_order function is in the back_of_house module , so we can use super to go to
// the parent module of back_of_house , which in this case is crate , the root . From there , we
// look for serve_order and find it . Success! we think the back_of_house module and the serve_order
// functions are likely to stay in the same relationship to each other and get moved together should
// we decide to reorganiwe the crate's module tree . Therefore , we used super so we'll have fewer
// places to updates code in the furure if this code gets moved to a different module .
