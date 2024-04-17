use crate::mealtime::breakfast::Breakfast as ololo;
pub use crate::temporal::season::Season;

pub mod mealtime;
pub mod temporal;

pub mod front_of_house;

pub fn deliver_order() {
    println!("The order has been delivered!");
}

pub mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        println!("Fixing in progress!");
        super::deliver_order();
        //deliver_order(); // ERROR: out of this scope!
    }

    fn cook_order() {}

    // structure
}

pub fn eat_at_restaurant() {
    println!("EATING");

    // Order a breakfast in the summer with Rye toast
    let meal = ololo::new(String::from("Rye")); // ololo is the alias of Breakfast struct

    println!("I'd like {} toast please", meal.get_toast());
}
