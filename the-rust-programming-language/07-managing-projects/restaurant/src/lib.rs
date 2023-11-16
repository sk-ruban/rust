// modules can hold child modules, functions, structs, enums, etc.
// modules help group related definitions together
// entire module tree is rooted under crate

// crate
// └── front_of_house
// ├── hosting
// │   ├── add_to_waitlist
// │   └── seat_at_table
// └── serving
// ├── take_order
// ├── serve_order
// └── take_payment

// paths can either be relative or absolute (preferred)
// absolute paths start with crate
// relative paths start with self,super (works like ..), or a specific identifier
// all items are private to parent modules by default
// but items in child modules can use ancestor modules
// enums are public by default

// front_of_house does not need to be public since eat_at_restaurant is a sibling
mod front_of_house;

fn deliver_order() {}

// use helps to shorten relative paths in the scope that use occurs
// re-exporting - bringing an item into scope and making it available
pub use crate::front_of_house::hosting;

// rename with alias
use std::fmt::Result;
use std::io::Result as IoResult;

// shortens the code by collapsing the common part of the path
use std::{cmp::Ordering, io};

// brings all public items defined in a path to scope
use std::collections::*;

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // super goes one parent module up, back to crate
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        // private field
        seasonal_fruit: String,
    }

    impl Breakfast {
        // we need this function to construct an instance of a private field
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path - shortened by use
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

