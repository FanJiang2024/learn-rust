#[test]
fn higher_func_test() {

  fn func_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
  }

  fn add_self(x: i32) -> i32  {
    x + x
  }

  fn mul_self(x: i32) -> i32 {
    x * x
  }

  assert_eq!(func_twice(add_self, 2), 8);
  assert_eq!(func_twice(mul_self, 4), 256);

  assert_eq!((1..=5).into_iter().map(|x| x + 10).collect::<Vec<i32>>(), vec![11, 12, 13, 14, 15]);
  assert_eq!((1..=10).into_iter().filter(|x| x % 2 == 0).collect::<Vec<i32>>(), vec![2,4,6,8, 10]);

  assert_eq!((1..=10).into_iter().fold(0, |acc, x| x + acc), 55);
}