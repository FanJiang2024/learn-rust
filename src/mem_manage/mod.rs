#[test]
fn copy_move_test() {
    let c1 = 1;
    let c2 = c1;    // copy; c1 is basic type
    println!("c1: {c1}, c2: {c2}");

    let s1 = String::from("value");
    let s2 = s1;    // move; s1 moves ownership to s2
    println!("s1 is gone. Now s2 holds ownership: {s2}"); 

    use super::utils::first_word;
    assert_eq!(first_word("hello world"), "hello");
    assert_eq!(first_word("We are the world"), "We");
}