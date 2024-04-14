#[test]
fn lifetimes_func_test() {
    fn longest_str<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        if s1.len() >= s2.len() {
            s1
        } else {
            s2
        }
    }

    // 这个函数更通用
    fn longest_str_pro<'a, 'b, 'out>(s1: &'a str, s2: &'b str) -> &'out str
    where
        // 'out 的生命周期是 'a 和 'b 的交集
        'a: 'out, // 'a 的生命周期 >= 'out 的生命周期
        'b: 'out, // 'b 的生命周期 >= 'out 的生命周期
    {
        if s1.len() >= s2.len() {
            s1
        } else {
            s2
        }
    }

    assert_eq!(longest_str("hello", "hello_world"), "hello_world");

    let s1 = "hello";
    {
        let mut s2 = "s2222222222";
        assert_eq!(longest_str(s1, s2), s2);

        s2 = "s2";
        assert_eq!(longest_str_pro(s1, s2), s1);
    }
}

#[test]
fn lifetimes_struct_test() {
    struct MyString<'a> {
        text: &'a str,
    }

    trait MyStringTrait<'a> {
        fn get_length(&self) -> usize;

        fn modify<'b>(&mut self, val: &'b str)
        where
            'b: 'a;
    }

    impl<'a> MyStringTrait<'a> for MyString<'a> {
        fn get_length(&self) -> usize {
            self.text.len()
        }

        fn modify<'b>(&mut self, val: &'b str)
        where
            'b: 'a,
        {
            self.text = val;
        }
    }

    let s = "hello".to_string();
    let mut my_string = MyString {
        text: s.as_str()
    };
    assert_eq!(my_string.text, s.as_str());
    my_string.modify("world");
    assert_eq!(my_string.text, "world");


    #[derive(Debug)]
    struct StringHolder {
        data: String
    }

    impl StringHolder {
        fn get_ref(&self) -> &String {
            &self.data
        }
    }

    // move!!!
    fn print_string_holder(h: StringHolder) {
        println!("StringHolder is :{:?}", h);
    } 

    let string_holder = StringHolder {
        data: "breakfast".to_owned()
    };
    let s = string_holder.get_ref();
    println!("{s}");

    println!("{:?}", string_holder);
    println!("{:?}", string_holder);

    print_string_holder(string_holder);


}
