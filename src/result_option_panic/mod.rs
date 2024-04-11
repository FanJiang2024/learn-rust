
#[test]
fn result_option_panic_test() {
    use std::panic::catch_unwind;
    use std::fmt::Display;
    use std::error::Error;

    // Result
    #[derive(Debug, PartialEq)]
    struct DivideErr;

    impl Display for DivideErr {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Cannot divided by 0!")
        }
    }

    impl Error for DivideErr {}

    fn divide(a: i32, b: i32) -> Result<f64, DivideErr> {
        if b == 0 {
            return Err(DivideErr{});
        }
        return Ok(a as f64 / b as f64);
    }

    assert_eq!(divide(1, 2), Ok(0.5));
    assert_eq!(divide(9, 0), Err(DivideErr{}));


    // Option
    fn find_element_by_value(arr: &[i32], target: i32) -> Option<usize> {
        for (index, item) in arr.iter().enumerate() {
            if *item == target {
                return Some(index);
            }
        }
        None
    }

    let arr = (1..=10).into_iter().collect::<Vec<_>>();
    assert_eq!(find_element_by_value(&arr, 5), Some(4));
    assert_eq!(find_element_by_value(&arr, 222), None);

    // Panic

    // #[should_panic]
    fn divide_pro(a: i32, b: i32) -> f64 {
        if b == 0 {
            panic!("{}", DivideErr{});
        }
        a as f64 / b as f64
    }
    assert_eq!(divide_pro(1, 5), 0.2);

    let result = catch_unwind(|| divide_pro(3, 0));
    assert!(result.is_err());


}
