#[test]
fn if_match_test() {
    // if 的表达能力不如match
    let num = 10;
    if num < 10 {
        println!("num less than 10");
    } else if num == 10 {
        println!("num is 10");
    } else {
        println!("num more than 10");
    }

    match num {
        10 => println!("num is 10"),
        _ => println!("num is others"),
    }

    match num {
        0..=9 => println!("num is more tha"),
        n if n % 2 == 0 => println!("num can be divided by 2"),
        10 | 11 | 12 => println!("num is 10 or 11 or 12"),
        _ => println!("num is others")
    }

    let res= match num {
        0..=9 => "!!!",
        _ => "~~~",
    }.repeat(3);
    println!("res: {}", res);
}