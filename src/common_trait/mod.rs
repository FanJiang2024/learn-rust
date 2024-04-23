#[test]
fn common_trait_test() {

    #[derive(Debug, Clone, Copy)]
    enum Color {
        Yellow,
        Black,
        Blue
    }

    impl PartialEq for Color {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Color::Black, Color::Black) => true,
                (Color::Blue, Color::Blue) => true,
                (Color::Yellow, Color::Yellow) => true,
                _ => false
            }
        }
    }

    #[derive(Debug, Clone)]
    struct User {
        // Rust中的所有包含堆内存或其他资源的类型（文件、网络套接字等）都不能实现Copy trait
        // 对于包含堆内存或其他资源的类型来说，复制不仅仅是复制其在内存中的位。例如，对于 String 类型，复制意味着需要分配新的堆内存并复制数据，这是一个昂贵的操作；对于其他类型的资源，复制这一操作甚至在操作系统层面都是不允许的
        name: String,
        age: usize,
        color: Color
    }

    impl PartialEq for User {
        fn eq(&self, other: &Self) -> bool {
            self.age == other.age && self.name == other.name && self.color ==  other.color
        }
    }

    let tom = User { name: "Tom".to_owned(), age: 23, color: Color::Black };
    println!("{:?}", tom);

    // let tom1 = tom;
    let tom2 = tom.clone();

    // println!("{:?}", tom1);
    assert_eq!(tom, tom2);
    
}