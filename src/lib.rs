// Making Structs and Enums Public
// We can also use pub to designate structs and enums as public , but there are a few extra details
// If we use pub before a struct definition , we make the struct public but the struct's fields will
// still be private . We can make each field public or not on a case-by-side basis .
// We've defined a public back_of_house::breakfast struct with a public toast field but a private
// seasonal_fruit field . This models the case in a restaurant where the customer can pick the type
// of bread that comes with a meal , but the chef decides which fruit accompanies the meal based on
// what's in season and in stock .
// The available fruit changes quickly , so customers can't choose the fruit or even see which fruit
// they'll get .

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
    //
}
