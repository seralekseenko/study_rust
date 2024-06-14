mod my_vectors;
mod my_strings;

fn main() {
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

// fn main() {
//     strings_test();
//     // vectors_test();
// }

fn strings_test() {
    println!("An empty string: {}", my_strings::create_empty_string());
    println!("Some string: {}", my_strings::get_string_from_literal("it was string literal"));
    my_strings::modify_empty_string();
    my_strings::print_modified_strings();
    my_strings::use_plus_operator_for_string();
    my_strings::use_format_macros();
    my_strings::try_get_symbol_from_string();
}

fn vectors_test() {
    my_vectors::get_some_vec();
    my_vectors::test_some_vectors();
    my_vectors::try_access_and_modify_vec();
    my_vectors::iterate_through_vec();
    my_vectors::try_vec_with_enum();
}
