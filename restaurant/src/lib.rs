mod front_of_house;//this, with the ; looks for that module's content in a file named "front_of_house.rs"

use crate::front_of_house::hosting;

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        crate::front_of_house::serving::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {//struct is public
        pub toast: String,
        seasonal_fruit: String,//this is private and not available to be called above the back_of_house module
    }

    impl Breakfast {
        pub fn summer(toast: &str)-> Breakfast {//will return a Breakfast struct
            Breakfast {//this initialization of the Breakfast structure is the last expression so this is what is returned
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches"),
            }

        }
    }

    pub enum Appetizer {
        Soup,//in an enum, all the enum's variants become public because of "pub"
        Salad,
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();//this doesn't build without "pub" because
    //the child modules to front_of_house are private and this item at the level of the parent can't
    //see inside private child modules

    hosting::add_to_waitlist();

    //Order breakfast in the summer with rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    //Change bread for the order
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please.", meal.toast);

    //this next line won't compile, trying to use something private
    //meal.seasonal_fruit = String::from("Guava");

    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}