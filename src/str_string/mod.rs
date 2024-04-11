#[test]
fn str_string_test() {
    let s1 = "hello".to_owned();
    let s2 = "hello".to_string();
    let s3 = String::from("hello");
    assert_eq!(s1, s2);
    assert_eq!(s1, s3);

    use super::utils::{print_str, print_string};

    print_str(&s1);
    print_string(s1);
}