// Making Structs and Enums Public

// In contrast , if we make an enum public , all of its variants are then public .
// We only need the pub before the enum keyword :

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
// Designating an enum as public makes all its variants public.
