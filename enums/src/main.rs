#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
#[allow(unused)]
enum Message {
    Quit,
    Move { x: isize, y: isize },
    Write(String),
    ChangeColor(isize, isize, isize),
}

impl Message {
    fn call(&self) {
        println!("Fuck the gusse! Write is: {:?}", &self);
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
    Unnamed,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match &self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                // ← here we define the var name "state" for UsState type
                // this match automaticaly extract a UsState from Quarter
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }
}

fn use_option_match_plus_1(num: Option<usize>) -> Option<usize> {
    match num {
        None => None,
        Some(value) => Some(value + 1),
    }
}

fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    //let five = Some(5);
    //let six = use_option_match_plus_1(five);
    //println!("Six is: {}", six.expect("Six should not be empty!"));
    //let none = use_option_match_plus_1(None);
    //println!("None is none: {}", none.is_none());

    //let coin_p = Coin::Penny;
    //println!("value of coin_p is: {}", coin_p.value_in_cents());
    //let coin_q = Coin::Quarter(UsState::Unnamed);
    //println!("value of coin_q is: {}", coin_q.value_in_cents());
    //let four = IpAddrKind::V4(127, 0, 0, 1);
    //let six = IpAddrKind::V6(String::from("::1"));
    //
    //route(four);
    //route(six);
    //
    //let message = Message::Write(String::from("hello"));
    //message.call();
    //
    //let k = 10;
    //let some = Some(4).unwrap_or_else(|| 2 * k);
    //assert_eq!(some, 4);
    //println!("some: {}", some);
    //let none = None.unwrap_or_else(|| 2 * k);
    //assert_eq!(none, 20);
    //println!("none: {}", none);
}

fn route(ip_kind: IpAddrKind) {
    dbg!(ip_kind);
}
