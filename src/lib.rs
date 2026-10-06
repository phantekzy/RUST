// Bringin Paths into Scope with the use Keyword :
//
// It might seem like the paths we've written to call functions so fat are inconveniently long and
// repetitive .
// We had to chose the absolute or relative path every time we wanted to call a function .
// Fortunately , there is a way to simplify this process . we can bring a path into a scope once and
// then call the items in that path as if they're local items with the use Keyword .

mod front_of_house {
    pub mod hostin {
        pub fn add_to_waitlist() {}
    }
}
// Using the 'use' Keyword
use crate::front_of_house::hostin; // Bringing a module into scope with use

pub fn eat_at_restaurant() {
    hostin::add_to_waitlist();
}
