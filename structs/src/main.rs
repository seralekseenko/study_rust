fn main() {
    let test_user = Uzver {
        email: String::from("ololo@trololo.com"),
        active: true,
        username: String::from("Trololony"),
        sign_in_count: 1,
    };
}

struct Uzver {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,


}
