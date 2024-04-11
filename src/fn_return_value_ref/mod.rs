
#[test]
fn fn_return_value_ref_test() {
    use std::ptr::addr_of;

    // 一般函数中返回Copy类型比返回non—copy类型性能要更好，
    // 因为Copy类型不涉及内存的转移和释放

    // return cpoy type
    fn return_copy_type(n: i32) -> *const i32 {
        return addr_of!(n);
    }
    let n = 32;
    assert_ne!(addr_of!(n), return_copy_type(n));

    // return non-copy type
    fn return_non_copy_type() -> String {
        "hello".to_string()
    }
    let s = return_non_copy_type();
    assert_eq!(s, "hello".to_string());

    fn return_static_ref() -> &'static str {
        let s = "hello";
        &s
    }
    assert_eq!("hello", return_static_ref());
}