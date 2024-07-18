pub mod lib {
    pub fn println() {
        println!("Hello lib");
    }
}

pub use front_of_house::hosting;

pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }
    pub mod serving {
        pub fn take_order() {
            super::super::eat_at_restaurant();
        }
        pub fn serve_order() {}
        pub fn take_payment() {}
    }
}

use back_of_house::Breakfaset;
pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfaset::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please.", meal.toast,);
    println!("{}", meal.get_seasonal_fruit());

    let order1 = back_of_house::Appetizer::Salad;
    println!("{:?}", order1);

    let meal = Breakfaset::summer("no");
    println!("{:?}", meal.toast);
}

pub fn deliver_order() {}

pub mod back_of_house {
    use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }

    pub fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfaset {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfaset {
        pub fn summer(toast: &str) -> Breakfaset {
            Breakfaset {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    impl Breakfaset {
        pub fn get_seasonal_fruit(self) -> String {
            self.seasonal_fruit
        }
    }

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
