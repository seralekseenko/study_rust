#[derive(Debug)]
struct Rectangle {
    width: usize,
    height: usize,
}

fn main() {
    let scale: usize = 123;
    let rect1 = Rectangle {
        width: dbg!(30 + scale),
        height: 50,
    };

    dbg!(&rect1);

    //println!("Rect is: {:#?}", rect1);
}

//fn area(rectangle: &Rectangle) -> usize {
//    rectangle.width * rectangle.height
//}
