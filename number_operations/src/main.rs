//use std::fmt;


fn main() {
    // addition
    let sum = 5 + 10;
    println!("sum = 5 + 10 → {} {} {1}", sum, "ololo");
    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference = 95.5 - 4.3 → {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("product = 4 * 30 → {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("quotient =  56.7 / 32.2 → {}", quotient);
    let floored = 2 / 3; // Results in 0
    println!("floored = 2 / 3 → {}", floored);

    // remainder
    let remainder = 43 % 5;
    println!("remainder = 43 % 5 → {}", remainder);

    println!(" {Z} {z} {heart_eyed_cat} ", Z = 'Z', z = 'z', heart_eyed_cat = '😻');

    //println!("u4:    {}", u4::MAX); // impposible type
    println!("u8::MAX - 1:    {}", (u8::MAX - 1));
    println!("u16::MAX:   {}", u16::MAX);
    println!("u32::MAX:   {}", u32::MAX);
    println!("u64::MAX:   {}", u64::MAX);
    println!("usize::MAX: {}", usize::MAX);
    println!("u128::MAX:  {}", u128::MAX);
    //println!("f8:    {}", f8::MAX);
    //println!("f16:   {}", f16::MAX);
    println!("f32::MAX:         {}", f32::MAX);
    println!("f32::MAX - 1.5:   {}", (f32::MAX - 1.5f32));
    println!("f64::MAX:         {}", f64::MAX);
    println!("f64::MAX - 1.5:   {}", f64::MAX - 1.5f64);
    println!("f32::MAX - f32::MIN:  {}", (f32::MAX - f32::MIN));
    println!("f32::MIN - 1.0:  {}", f32::MIN - 1.0);

}
