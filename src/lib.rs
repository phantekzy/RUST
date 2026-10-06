// Making Structs and Enums Public

// In contrast , if we make an enum public , all of its variants are then public .
// We only need the pub before the enum keyword :

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from("toast"),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // The next line won't compile if we uncoment it ; we're not allowd to see or modify
    // seasonal_fruit that comes with the meal .
    // meal.seasonal_fruit = String::from("Bananas");
    // Because the toast field in the back_of_house::Breakfast struct is public , in
    // eat_at_restaurantwe can write and read the toast field using do notaion .
    // Notice that we can't use the seasonal_fruit field in eat_at_restaurant Because
    // seasonal_fruitis private .
    // Also, Because back_of_house::Breakfast has a private field , the struct needs to provide a
    // public associated function that constructs an instance of Breakfast (we've named it summer) .
    // If Breakfast didn't have such a funtion , we couldn't create an instance of Breakfast in eat_at_restaurant
    // Because we couldn't set the value of the private seasonal_fruit field in eat_at_restaurant .
}
