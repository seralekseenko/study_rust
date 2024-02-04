fn main() {
    run_test_strings_slices();
    run_test_array_slices();
}

fn run_test_array_slices() {
    let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let array_slice = &array[3..6];
    assert_eq!(array_slice, &[3, 4, 5]);

}

fn run_test_strings_slices() {
    let my_string = String::from("SOME TEST TEXT IN STRING");
    println!("{}", my_string);
    // `first_word` works on slices of `String`s, whether partial or whole
    let word1 = first_word(&my_string[0..6]);
    println!("word1: {}", word1);
    let word2 = first_word(&my_string[..]);
    println!("word2: {}", word2);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word3 = first_word(&my_string);
    println!("word3: {}", word3);
    let my_string_literal = "It is a literal. What does it mean?";
    println!("my_string_literal: {}", my_string_literal);
    // `first_word` works on slices of string literals, whether partial or whole
    let word4 = first_word(&my_string_literal[0..6]);
    println!("word4: {}", word4);
    let word5 = first_word(&my_string_literal[..]);
    println!("word5: {}", word5);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word6 = first_word(my_string_literal);
    println!("word6: {}", word6);
}    

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
