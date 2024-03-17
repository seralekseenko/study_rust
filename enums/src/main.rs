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

fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));

    route(four);
    route(six);

    let message = Message::Write(String::from("hello"));
    message.call();

    let k = 10;
    let some = Some(4).unwrap_or_else(|| 2 * k);
    assert_eq!(some, 4);
    println!("some: {}", some);
    let none = None.unwrap_or_else(|| 2 * k);
    assert_eq!(none, 20);
    println!("none: {}", none);
}

fn route(ip_kind: IpAddrKind) {
    dbg!(ip_kind);
}
