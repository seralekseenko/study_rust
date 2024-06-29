

#[allow(unused)]
mod my_vectors;
#[allow(unused)]
mod my_strings;
mod my_hash_maps;
mod median_and_mode;
mod pig_latin;

fn main() {

    //test_hash_maps();
    // strings_test();
    // vectors_test();

}


#[allow(dead_code)]
fn test_hash_maps() {
    my_hash_maps::test_hash_map_with_str_nums();
    my_hash_maps::try_get_in_loop();
    my_hash_maps::test_ownership();
    my_hash_maps::test_replace_value();
    my_hash_maps::counter_with_map();
}

#[allow(dead_code)]
fn strings_test() {
    println!("An empty string: {}", my_strings::create_empty_string());
    println!("Some string: {}", my_strings::get_string_from_literal("it was string literal"));
    my_strings::modify_empty_string();
    my_strings::print_modified_strings();
    my_strings::use_plus_operator_for_string();
    my_strings::use_format_macros();
    my_strings::try_get_symbol_from_string();
    my_strings::play_with_indexes();
}

#[allow(dead_code)]
fn vectors_test() {
    my_vectors::get_some_vec();
    my_vectors::test_some_vectors();
    my_vectors::try_access_and_modify_vec();
    my_vectors::iterate_through_vec();
    my_vectors::try_vec_with_enum();
}
