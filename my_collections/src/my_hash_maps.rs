use std::collections::HashMap;
use crate::my_strings::create_empty_string;


fn print_hash_map(hash_map: HashMap<&str, usize>) {
    println!("{:?}", hash_map);
}

fn create_and_fill_hash_map() -> HashMap<&'static str, usize> {
    let mut result = HashMap::new();
    let entries: [(&str, usize); 3] = [
        ("FirstGet", 99),
        ("SecondGet", 100),
        ("ThirdGet", 101),
    ];
    for (key, value) in entries {
        result.insert(key, value);
    }
    return result;
}

pub(crate) fn test_hash_map_with_str_nums() {
    let string_nums: HashMap<&str, usize> = create_and_fill_hash_map();

    print_hash_map(string_nums);
}

pub(crate) fn try_get_in_loop() {
    println!("Try to use get() method from HashMap in the for loop");
    let string_nums: HashMap<&str, usize> = create_and_fill_hash_map();
    let keys = string_nums.keys();
    let mut prnt_str = create_empty_string();
    prnt_str.push('{');
    for key in keys {
        if let Some(value) = string_nums.get(key) {
            prnt_str.push_str(&format!("[Key is: {key}, Obtained value: {value}] "));
        }
    }
    prnt_str.push('}');
    println!("{}", prnt_str);
}