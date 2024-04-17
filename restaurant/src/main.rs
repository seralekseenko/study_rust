use restaurant::eat_at_restaurant;
use restaurant::{back_of_house, front_of_house};
//use restaurant::front_of_house;

fn main() {
    println!("Hello, world!");
    eat_at_restaurant();

    let season_fruit = restaurant::Season::get_some_season_fruit_name();
    println!("Season fruit is {}", season_fruit);
    front_of_house::hosting::add_to_waitlist();

    //restaurant::deliver_order();
    back_of_house::fix_incorrect_order();
    //eat_at_restaurant();
}
