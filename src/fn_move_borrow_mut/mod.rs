#[test]
fn fn_move_borrow_mut_test() {
    // borrow
    fn print_string(s: &String) {
        println!("s is: {s}");
    }
    let s = "ss".to_string();
    print_string(&s);

    fn print_num_string(x: i32, s: String) {
        println!("x is {x}");   // copy
        println!("s is {}", s.to_uppercase()); // move
    }
    let a = 111111;
    let s = "hello world".to_string();
    print_num_string(a, s);
    println!("a is {a}");
    // println!("s is {s}");   


    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    // move
    fn print_point(p: Point) {
        println!("point is {:?}", p);
    }
    let p = Point { x:0, y: 0 };
    print_point(p);
    // println!("p is {:?}", p);

    // immutable borrow
    fn print_point_ref(p: &Point) {
        println!("point is {:?}", p);
    }
    let p = Point{ x: 111, y: 2222 };
    print_point_ref(&p);
    println!("p is: {:?}", p);

    let mut mutable_point = Point {
        x: 12,
        y: 24,
    };

    // mutable borrow
    fn modify_point(p: &mut Point) {
        p.x += 10;
        (*p).y += 100;
    }
    modify_point(&mut mutable_point);
    assert_eq!(mutable_point, Point{ x: 22, y: 124 });

}