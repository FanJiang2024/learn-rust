#[test]
fn box_test() {
    struct Point {
        x: i32,
        y: i32
    }
    let boxed_point = Box::new(Point{ x: 2, y: 3 });
    println!("x: {}, y: {}", boxed_point.x, boxed_point.y);
    // let mut p = boxed_point; // moved!!!
    println!("x: {}, y: {}", boxed_point.x, boxed_point.y);

    let mut boxed_number = Box::new(32);
    println!("{}", boxed_number);
    
    *boxed_number += 20;
    println!("{}", boxed_number);
    
}