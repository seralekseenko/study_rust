pub(crate) fn get_some_vec() -> Vec<usize> {
    let some_vector: Vec<usize> = Vec::new();
    return some_vector;
}

pub(crate) fn get_vec_with_i32() -> Vec<i32> {
    let i32_vec = vec![1, 2, 3];
    return i32_vec;
}

pub(crate) fn test_some_vectors() {
    let mut usize_vec = get_some_vec();
    println!("Empty vector: {:?}", usize_vec);
    usize_vec.push(0);
    println!("Is empty vector after push: {}", usize_vec.is_empty());
    println!("Already is not the empty vector: {:?}", usize_vec);

    let i32_vec = get_vec_with_i32();
    println!("Vector with numbers i32: {:?}", i32_vec);

    let index: usize = 1;
    let some_element: &i32 = &i32_vec[index];
    eprintln!("i32_vec index {} contains {}", &index, &some_element);
    let the_same_element: Option<&i32> = i32_vec.get(index);
    match the_same_element {
        Some(the_same_element) => println!("The the_same_element is {}", the_same_element),
        None => println!("There is no the_same_element."),
    }
}

pub(crate) fn try_access_and_modify_vec() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0].clone();

    v.push(6);

    println!("The first element is: {}", first);
}

pub(crate) fn iterate_throughth_vec() {
    let mut v = vec![100, 32, 57, 66, 33, 55];
    for element in &mut v {
        *element *= 100; // * is the dereference operator
    }
    // just print without mutations
    for element in &v {
        println!("{}", element);
    }
}

#[derive(Debug)]
enum SpreadsheetCell {
    Id(usize),
    FloatPrecision(f64),
    SomeText(String),
    Boolean(bool),
}

pub(crate) fn try_vec_with_enum() {
    let mut one_row = vec![
        SpreadsheetCell::Id(1),
        SpreadsheetCell::FloatPrecision(1.1),
        SpreadsheetCell::SomeText(String::from("First row in a table")),
        SpreadsheetCell::Boolean(true),
    ];
    one_row.push(SpreadsheetCell::Id(2));
    println!("First row of vector contains enum: {:#?}", &one_row);

    for el in &one_row {
        println!("Element: {:?}", el);
    }
}
