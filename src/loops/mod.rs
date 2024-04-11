#[test]
fn loops_test() {
    let mut num = 0;
    // loop 创建一个无限循环，可用break跳出
    loop {
        if num >= 10 {
            break;
        };
        // println!("num is {num}");
        num += 1;
        // std::thread::sleep(std::time::Duration::from_secs_f32(0.2));
    }

    assert_eq!(num, 10);

    while num > 0 {
        num -= 1;
    }
    assert_eq!(num, 0);

    for i in 0..10 {
        num = i;
    }
    assert_eq!(num, 9);

    for i in 0..=10 {
        num = i;
    }
    assert_eq!(num, 10);

    // break 只能跳出inner loop，不能跳出outer loop，因此这段代码会死循环；
    // loop {
    //     println!("outer");
    //     loop {
    //         println!("inner");
    //         break;
    //     }
    // }

    // loop with tag!!!
    // 利用break跳出指定循环
    'outer: loop {
        println!("outer");
        loop {
            println!("inner");
            break 'outer;
        }
    }

    let iter_numbers :Vec<_> = std::ops::RangeInclusive::new(0, 5).map(|x| x * x).collect();
    assert_eq!(iter_numbers, vec![0, 1, 4, 9, 16, 25]);

    let test = (0..5).map(|x| x + 10).collect::<Vec<i32>>();
    assert_eq!(test, vec![10, 11, 12, 13, 14]);
}
