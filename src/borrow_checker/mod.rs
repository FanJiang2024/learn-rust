#[test]
fn borrow_checker_test() {
  let mut hello = "hello".to_string();

  let h1 = &hello;
  let h2 = &hello;
  // many immutable borrowing exist in the same time.
  assert_eq!(h1, h2);

  let h3 = &mut hello;
  // only one mutable(exclusive) borrowing exists in the same time.
  assert_eq!(h3, "hello");

  let mut a = 10;
  let b = &a;
  assert_eq!(*b, 20);
  {
    // 程序在第21行使用到了b（a的不可变引用）
    // 程序执行到第18行时发现c（a的不可变引用）
    // 这时程序中同时存在a的两种矛盾引用，因此报错
    // 解决方案是：在声明a一种引用之前先用掉之前存在的a引用（例如在声明b之后立刻用掉）。
    let c = &mut a;
    *c = 20;
  }
  // assert_eq!(*b, 20);
  assert_eq!(a, 20);
}