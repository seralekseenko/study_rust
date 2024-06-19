pub(crate) fn create_empty_string() -> String {
    let mut empty_string = String::new();
    return empty_string;
}

pub(crate) fn get_string_from_literal(literal: &str) -> String {
    literal.to_string() // the same as String::from(literal)
}

pub(crate) fn modify_empty_string() {
    let mut some_string = create_empty_string();
    let s1 = "OLOLO";
    let s2 = "trololo";
    some_string.push_str(s1);
    some_string.push_str(" ");
    some_string.push_str(&s2);
    println!("Modified empty string: {}", some_string);
    println!("s1 was: {}", s1);
    println!("s2 was: {}", s2);
}

pub(crate) fn print_modified_strings() {
    let mut foo = String::from("foo");
    let bar = "bar";
    foo.push_str(bar);
    println!("foo is {}", foo);
    println!("bar is {}", bar);
}

pub(crate) fn use_plus_operator_for_string() {
    let s1 = String::from("Hello, ");
    let s2 = /*String::from(*/"world!"/*)*/;
    let s3 = s1 + /*&*/s2; // note s1 has been moved here and can no longer be used
    print!("use operator +, result, s3 is: {} ", s3);
    // print!("s1 is: {}", s1); // borrow error
    print!("s2 is: {} ", s2);
}

pub(crate) fn use_format_macros() {
    print!("{}", format!("\nFirst String: {}\nSecond string: {}\n",
                         String::from("test string"), String::from("test string 2")));
    print!("{}", format!("\nThird String: {}\nFourth string: {}\n",
                         "three", "four"));
}

pub(crate) fn try_get_symbol_from_string() {
    let string_object: String = String::from("тестова строка Українською мовою. And English words.");
    let result1 = &string_object.chars().nth(7).unwrap();
    println!("Symbol from String object: {}", result1);
    let string_literal = "шдшс String literal ідіс";
    println!("Symbol from String literal: {}", &string_literal[0..2]);
    let result2 = &string_object[15..27];
    println!("Slice from string_object: {}", result2);
}

pub(crate) fn play_with_indexes() {
    let s = String::from("тестова строка Українською мовою");


    // Визначаємо точні індекси символів
    let start = s.char_indices().nth(15).unwrap().0;  // Початок підстроки "Українською"
    let end = s.char_indices().nth(26).unwrap().0;    // Кінець підстроки "Українською"
    println!("Start: {}, End: {}", start, end);

    if let Some(sub) = s.get(start..end) {
        println!("{}", sub); // "Українською"

    } else {

    }


    let s2 = "тестова строка Українською мовою";

    // Визначаємо точні індекси символів
    let start2 = s2.char_indices().nth(15).unwrap().0;  // Початок підстроки "Українською"
    let end2 = s2.char_indices().nth(25).unwrap().0;    // Кінець підстроки "Українською"

    let sub2 = &s2[start2..end2];

    println!("{}", sub2); // "Українською"
}