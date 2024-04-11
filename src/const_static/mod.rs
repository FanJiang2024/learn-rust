// 使用static声明静态变量，必须注明变量类型
static  MY_STATIC: i32 = 32;
static mut MY_MUT_STATIC: i32 = 64;


#[test]
fn const_static_test() {
    // 使用const声明常量，必须注明常量类型
    const SECOND_HOUR :usize = 3_600;
    const SECOND_DAY : usize = 24 * SECOND_HOUR;

    println!("SECOND_DAY: {SECOND_DAY}");

    println!("MY_STATIC: {MY_STATIC}");

    unsafe {
        MY_MUT_STATIC = 128;
        println!("MY MUT STATIC: {MY_MUT_STATIC}");
    }
}