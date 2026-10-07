mod front_of_house {
    pub mod hostin {
        pub fn add_to_waitlist() {}
    }
}
use self::front_of_house::hostin;

pub fn eat_at_restaurant() {
    hostin::add_to_waitlist();
}
