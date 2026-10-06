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
//
// Because we made the Appetizer enum public , we can use the Soup and Salad variants in eat_at_restaurant .
// Enums aren't very usefull unless their variants are public ; it would be annoying to have to
// annotate all enums varants with pub in every case , so the default for enums variants is to be
// public .
