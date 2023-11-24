use rand::Rng;


fn main() {
    let number = get_random_number(0, 11);

    if number < 5 {
        println!("number was lower than 5: {number}");
    } else if number == 5 {
        println!("number was equal to 5!: {number}");      
    } else {
        println!("number was more than 5: {number}");
    }

    let is_divisible = if number % 2 == 0 {
                     "number is divisible by 2"
		 } else {
		     "number is not divisible by 2"
		 };
    println!("The {is_divisible}");


}

fn get_random_number(from: i32, to: i32) -> i32 {
   return rand::thread_rng().gen_range(from..=to); 


}
