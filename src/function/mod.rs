#[test]
fn function_test() {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    assert_eq!(3, add(1, 2));

    fn try_to_change_num(mut n: i32) {
        n += 10; // failed!!!
    }
    let a = 1;
    try_to_change_num(a);
    assert_eq!(a, 1);

    fn modify_num(n: &mut i32) {
        *n += 10;
    }
    let mut a = 10;
    modify_num(&mut a);
    assert_eq!(a, 20);

    #[derive(Debug, Clone, Copy)]
    struct Person {
        size: u8,
        age: u8,
    }

    let Tom = Person {
        size: 11,
        age: 11,
    };
    fn print_person(p: Person) {
        println!("Person: {:?}", p);
    }
    print_person(Tom);  // the ownership of Tom moved to print_person.
    println!("Tom age: {}", Tom.age);   // the Person struct implements the copy trait.
}
