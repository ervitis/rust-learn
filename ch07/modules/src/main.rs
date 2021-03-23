mod lib;

use lib::front_of_house;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }
}

fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    println!("{}", meal.toast)
}

fn main() {
    front_of_house::hosting::add_to_waitlist();
    eat_at_restaurant()
}
