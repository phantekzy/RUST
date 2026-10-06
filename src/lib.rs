// Bringin Paths into Scope with the use Keyword :
//
// It might seem like the paths we've written to call functions so fat are inconveniently long and
// repetitive .
// We had to chose the absolute or relative path every time we wanted to call a function .
// Fortunately , there is a way to simplify this process . we can bring a path into a scope once and
// then call the items in that path as if they're local items with the use Keyword .

// Adding use and a path in a scope is similar to creating a symbolic link in the filesystem .
// By adding use crate::front_of_house::hostin in the crate root , hostin is now a valid name in
// that scope . just as though the hostin module has been defined in the crate root .
// Paths brought into scope with use also check privacy , like any other paths .
// Specifying a relative path with use is slightly different . instead of starting from a name in
// current scope , we must start thte path given to use with the Keyword self .

mod front_of_house {
    pub mod hostin {
        pub fn add_to_waitlist() {}
    }
}
// Brining module into scope with use and a relative path starting with self .
use self::front_of_house::hostin;

pub fn eat_at_restaurant() {
    hostin::add_to_waitlist();
}

// Note that using self in this way might not be necessary in the future ; it's an inconsistency in
// the language that Rust developers are working to eliminate
