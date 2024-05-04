#[test]
fn iter_loop() {
    fn sum_with_loop(arr: &Vec<i32>) -> i32 {
        let mut sum = 0;
        for i in arr {
            sum += i;
        }
        sum
    }

    fn sum_with_iter(arr: &Vec<i32>) -> i32 {
        arr.iter().sum()
    }

    let arr = (1..=10000).collect::<Vec<i32>>();
    assert_eq!(sum_with_loop(&arr), sum_with_iter(&arr));
}

#[test]
fn connection_test() {
    // IntoIterator、Iterator、iter之间的关系

    let s = "hello world!".to_string();
    let upper_s = s
        .chars()
        .map(|c| c.to_ascii_uppercase())
        .collect::<String>();
    assert_eq!(upper_s, "HELLO WORLD!".to_owned());
}

#[test]
fn test() {
    // iter into_iter iter_mut
    let nums = vec![1, 2, 3, 4, 5];

    // iter: 只读引用，过多使用引用不利于项目之后的维护工作
    assert_eq!(nums.iter().sum::<i32>(), 15);
    assert_eq!(nums, nums);

    let mut arr = vec![1, 2, 3, 4, 5];
    // iter_mut: 性能很差，慎用！！！
    arr.iter_mut().for_each(|x| *x *= 2);
    assert_eq!(arr, vec![2, 4, 6, 8, 10]);

    // into_iter: move!!!
    // 推荐使用
    assert_eq!(nums.into_iter().sum::<i32>(), 15);
}
