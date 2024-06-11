mod my_vectors;

fn main() {
    vectors_test();
}

fn vectors_test() {
    my_vectors::get_some_vec();
    my_vectors::test_some_vectors();
    my_vectors::try_access_and_modify_vec();
    my_vectors::iterate_through_vec();
    my_vectors::try_vec_with_enum();
}
