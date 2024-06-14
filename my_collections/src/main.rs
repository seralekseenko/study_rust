mod my_vectors;
mod my_strings;
mod my_hash_maps;


fn main() {
    strings_test();
    // vectors_test();
}

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

fn vectors_test() {
    my_vectors::get_some_vec();
    my_vectors::test_some_vectors();
    my_vectors::try_access_and_modify_vec();
    my_vectors::iterate_through_vec();
    my_vectors::try_vec_with_enum();
}
