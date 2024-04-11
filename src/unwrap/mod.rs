
#[test]
fn unwrap_test() {
  use std::num::ParseIntError;

  fn find_first_even(vec: &[i32]) -> Option<i32> {
    // 如果找不到程序不会继续向下执行，而是直接返回 None
    let num = vec.iter().find(|&&num| num % 2 == 0)?;
    Some(*num)
  }

  assert_eq!(find_first_even(&vec![1, 2, 3, 4]), Some(2));
  assert_eq!(find_first_even(&vec![1, 3, 5]), None);

  fn parse_int_from_str(s: &str) -> Result<i32, ParseIntError> {
    // ? 用于抛出可能的错误
    let num = s.parse::<i32>()?;
    Ok(num)
  }

  assert_eq!(parse_int_from_str("33"), Ok(33));
  assert!(parse_int_from_str("d").is_err());

  assert_eq!("56".parse::<i32>().unwrap(), 56);

}