const AY: &str = "ay";
const HAY: &str = "hay";

pub fn to_pig_latin(str: &str) -> String {
    return String::from("ololo");
}


#[cfg(test)]
mod tests {
    use super::*;

    // (input, output)
    const CONSONANTS: [(&str, &str); 3] = [
        ("first", "irst-fay"),
        ("hello", "ello-hay"),
        ("string", "tring-say"),
    ];

    const VOWELS: [(&str, &str); 3] = [
        ("apple", "apple-hay"),
        ("orange", "orange-hay"),
        ("umbrella", "umbrella-hay"),
    ];

    const MIXED: [(&str, &str); 3] = [
        ("first apple", "irst-fay apple-hay"),
        ("hello orange", "ello-hay orange-hay"),
        ("string umbrella", "tring-say umbrella-hay"),
    ];

    const NON_ASCII: [(&str, &str); 3] = [
        ("über", "über-hay"),
        ("ñandú", "ñandú-hay"),
        ("schön", "chön-say"),
    ];
    #[test]
    fn test_pig_latin_with_consonant() {
        let message = format!("Consonants FAILED!\nInputs: {:?}", CONSONANTS);
        universal_test(&CONSONANTS, message);
    }

    #[test]
    fn test_pig_latin_with_vowel() {
        let message = format!("Vowels FAILED!\nInputs: {:?}", VOWELS);
        universal_test(&VOWELS, message);
    }

    #[test]
    fn test_pig_latin_with_mixed_sentence() {
        let message = format!("Mixed sentences FAILED!\nInputs: {:?}", MIXED);
        universal_test(&MIXED, message);
    }

    #[test]
    fn test_pig_latin_with_non_ascii_characters() {
        let message = format!("Non ASCII chars FAILED!\nInputs: {:?}", NON_ASCII);
        universal_test(&NON_ASCII, message);
    }

    #[test]
    fn test_pig_latin_with_empty_string() {
        assert_eq!(to_pig_latin(""), "");
    }

    fn universal_test(tup_slice: &[(&str, &str)], message: String) {
        for &(input, expected_result) in tup_slice {
            assert_eq!(to_pig_latin(input), expected_result, "{}", message);
        }
    }

}

