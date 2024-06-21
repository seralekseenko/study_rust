use std::collections::HashMap;
use crate::my_strings::create_empty_string;


fn print_hash_map(hash_map: &HashMap<&str, usize>) {
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

    print_hash_map(&string_nums);
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

pub(crate) fn test_ownership() {
    println!("### Test ownership. ###");
    let mut some_map = create_and_fill_hash_map();
    let mut key = "Added string";
    let value = 999;
    some_map.insert(key, value);
    print_hash_map(&some_map);
    println!("Try print Key: {key}, Value: {value}");

    key = "Shaka maka";
    print_hash_map(&some_map);
    println!("Try print Key: {key}, Value: {value}");

}

pub(crate) fn test_replace_value() {
    println!("### Test replace and update. ###");
    let mut some_map = create_and_fill_hash_map();
    some_map.insert("T1", 1);
    print_hash_map(&some_map);
    some_map.insert("T1", 777);
    print_hash_map(&some_map);
    some_map.entry("T2").or_insert(888); // put if exist
    some_map.entry("T2").or_insert(5555); // put if exist
    print_hash_map(&some_map);

}

pub(crate) fn counter_with_map() {
    println!("### Test use map like a counter. ###");
    let text = "hello world wonderful world , boryambalo koryambalo";

    let mut map: HashMap<&str, usize> = HashMap::new();

    for word in text.split_whitespace() {
        // метод or_insert() повертає мутабельне посилання на value пов'язане з key
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    print_hash_map(&map);
}

