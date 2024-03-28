use restaurant::back_of_house;
use restaurant::eat_at_restaurant;
//use restaurant::front_of_house;

fn main() {
    println!("Hello, world!");
    restaurant::deliver_order();
    back_of_house::fix_incorrect_order();
    eat_at_restaurant();
}
