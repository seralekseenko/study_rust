fn main() {
    println!("Hello, world!");

    another_function(5, 'A', "ololo string");

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let from_five = five();
    println!("Value from five(): {from_five}");
    println!("Try five() inside println: {}", five());


    println!("get_str(): {}", get_str());
    println!("return_some_value(): {}", return_some_value());
}


fn another_function(number: u8, ch: char, string: &str) {
    println!("Another function. Args are: {number}, {ch}, {string}");
}

fn five() -> u8 {
    5
}

fn get_str() -> &'static str {
    "string from function"
}

fn return_some_value() -> u8 {
    return 111;
}
