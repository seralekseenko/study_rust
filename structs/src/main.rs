fn main() {
    let mut test_user = User {
        email: String::from("ololo@trololo.com"),
        active: true,
        username: String::from("Trololony"),
        _sign_in_count: 1,
    };
    println!("{}", test_user.email);
    test_user.email = String::from("ONOTHER@mail.com");

    println!("{}", test_user.email);

    let t_u_2 = build_u_short(String::from("EMAIL"), String::from("USERNAME"));
    println!(
        "t_u_2 name: {}\nt_u_2 mail: {}",
        t_u_2.username, t_u_2.email
    );

    let tu_3 = User {
        email: String::from("thirdEmail@mail"),
        ..test_user
    };
    println!(
        "tu_3 mail: {}\ntu_3 name: {}\ntu_3 active: {}",
        tu_3.email, tu_3.username, tu_3.active
    );

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
}

// Tuple struct
struct Color(usize, usize, usize);
struct Point(usize, usize, usize);

struct User {
    active: bool,
    username: String,
    email: String,
    _sign_in_count: usize,
}

fn _build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        _sign_in_count: 1,
    }
}

fn build_u_short(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        _sign_in_count: 1,
    }
}

// experimental sheet
//fn _create_new_user_from_old(
//    old: User,
//    email: String,
//    username: String,
//    active: bool,
//    sign_in_count: u64,
//) -> User {
//    User { ..old }
//}
