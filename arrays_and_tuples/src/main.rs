fn main() {
    let tup/*: (i32, f64, u8)*/ = (500, 6.4, 1);
    println!("This is the tupl: {:?}", tup);
    println!("0: {}, 1: {}, 2: {}", tup.0, tup.1, tup.2);

    let (int1, float, int2) = tup;
    println!("The values → int1: {int1}, float: {float}, int2: {int2}");

    let array1: &[u8] = &[1, 2, 3, 4, 5];
    // for remove WARNING about unused var → use _
    let _array2: [u8; 5] = [1, 2, 3, 4, 5];
    let array3: [u8; 5] = [1, 2, 3, 4, 5];
    println!("[{}, {}, {}, {}, {}]", array3[0], array3[1], array3[2], array3[3], array3[4]);
    println!("{:?}", array3);
    
    let strs1: [&str; 8] = ["a1", "b2", "c3", "d4", "e5", "f6", "g7", "j8"];
    println!("{:?}", strs1);

    let strings1: [String; 2] = ["ololo1".to_string(), "ololo2".to_string()];
    println!("{:?}", strings1);


}
