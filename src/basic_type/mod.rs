#[test]
fn basic_type_test() {
    // 进制的字面量
    let a1 = -125;  // 十进制
    let a2 = 0xFF;  // 十六进制
    let a3 = 0o13;  // 八进制
    let a4 = 0b10;  // 二进制
    println!("{a1}  {a2}  {a3}  {a4}");

    // Max Min
    println!("u32 Max: {}", u32::MAX);
    println!("u32 Min: {}", u32::MIN);
    println!("u32 is {} bytes", std::mem::size_of::<u32>());    // 4
    

    println!("i32 Max: {}", i32::MAX);
    println!("i32 Min: {}", i32::MIN);
    println!("i32 is {} bytes", std::mem::size_of::<i32>());    // 4

    println!("usize Max: {}", usize::MAX);
    println!("usize Min: {}", usize::MIN);
    println!("usize is {} bytes", std::mem::size_of::<usize>());    // 8

    println!("isize Max: {}", isize::MAX);
    println!("isize Min: {}", isize::MIN);
    println!("isize is {} bytes", std::mem::size_of::<isize>());    // 8

    println!("i64 is {} bytes", std::mem::size_of::<i64>());    // 8
    println!("u64 is {} bytes", std::mem::size_of::<u64>());    // 8
    
    // float
    let f1: f32 = 1.232323; // 尽量不用f32，因为变量大小边界很难确定；
    let f2: f64 = 9.9786;
    println!("rounded f1 is {:.2}, rounded f2 is {:.2}", f1, f2);
    // println!("f32 is {} bytes, f64 is {} bytes.", std::mem::size_of::<f32>(), std::mem::size_of::<f64>());
    println!("f32 is {} bytes", std::mem::size_of::<f32>());    // 4
    println!("f64 is {} bytes", std::mem::size_of::<f64>());    // 8

    // bool
    let is_ok = true;
    let can_ok = false;
    println!("is ok? {is_ok}, can ok? {can_ok}");
    println!("is_ok or can_ok: {}, is_ok and can_ok: {}", is_ok || can_ok, is_ok && can_ok);
    println!("bool is {} bytes", std::mem::size_of::<bool>());  // 1

    // char
    let char_c = 'C';
    let emo_char = '😀';
    println!("You get {char_c}, feel {emo_char}");
    println!("{}", emo_char as usize);  // 128512
    println!("{}", emo_char as i32);    // 128512

    println!("u8 Max: {}", u8::MAX);
    println!("u8 Min: {}", u8::MIN);
    println!("u16 Max: {}", u16::MAX);
    println!("u16 Min: {}", u16::MIN);
}