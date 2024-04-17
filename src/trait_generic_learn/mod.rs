#[test]
fn trait_generic_test() {
    use std::fmt::Display;

    trait Gretter {
        fn greet(&self)
        where
            Self: Display,
        {
            println!("Greet from {}", self);
        }
    }

    trait Hell {
        fn go_hell(&self)
        where
            Self: Display,
        {
            println!("{} goes to hell.", self);
        }
    }

    struct Person {
        name: String,
    }

    impl Display for Person {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.name)
        }
    }
    
    struct Animal {
        name: String,
    }
    
    impl Display for Animal {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.name)
        }
    }


    impl Gretter for Person {}
    impl Gretter for Animal {}

    impl Hell for Person {}
    impl Hell for Animal {}

    let tom = Person {
        name: "Tom".to_string(),
    };
    let dog = Animal {
        name: "dog".to_string(),
    };

    fn greeting<T: Gretter + Display>(a: &T) {
        a.greet()
    }

    greeting(&tom);
    greeting(&dog);

    fn greeting_from_person_amimal(a: &(impl Gretter + Display), b: &(impl Hell + Display)) {
        a.greet();
        b.go_hell();
    }

    greeting_from_person_amimal(&tom, &dog);
    greeting_from_person_amimal(&dog, &tom);

}


#[test]
fn test() {

    enum Lang {
        English = 0,
        Chinese = 1,
        Janpanese = 3,
    }

    // 行为
    trait Language {
        fn detect(&self, content: &str) -> Lang;
        fn translate(&self, target: Lang) -> String;
    }

    // 对象
    struct Material<T: Language> {
        language: T,
        material_body: String,
    }

    // 整合对象和行为
    impl<T: Language> Material<T> {
        fn get_content(&self) -> String {
            let lang = self.language.detect(&self.material_body);
            self.language.translate(lang)
        }
    }
}
