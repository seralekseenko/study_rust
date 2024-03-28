pub mod front_of_house {

    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    mod serving {
        //fn take_order() {}
        //
        //fn serve_order() {}
        //
        //fn take_payment() {}
    }
}

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
}

pub fn eat_at_restaurant() {
    println!("EATING");
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // or
    crate::front_of_house::hosting::seat_at_table();
    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // experiments
    back_of_house::fix_incorrect_order();
}
