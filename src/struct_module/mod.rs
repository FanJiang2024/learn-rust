#[test]
fn struct_test() {

    enum Flavor {
        Spicy,
        Fruity,
        Sweet
    }

    struct Drink {
        flavor: Flavor,
        price: f64,
    }

    impl Drink {
        // 关联变量; 必须声明类型
        const MAX_PRICE: f64 = 10.0;

        // methods
        fn buy(&self) -> bool {
            if self.price > Self::MAX_PRICE {
                println!("I am poor");
                return false;
            }
            println!("I can buy it");
            return true;
        }

        // 关联函数
        fn new(flavor: Flavor, price: f64) -> Self {
            Drink {
                flavor,
                price,
            }
        }
    }

    fn print_drink(drink: &Drink) {
        match drink.flavor {
            Flavor::Fruity => println!("Fruity"),
            Flavor::Sweet => println!("Sweet"),
            Flavor::Spicy => println!("Spicy")
        }
    }

    let sweet = Drink::new(Flavor::Sweet, 9.9);
    print_drink(&sweet);
    println!("{}", sweet.price);

    sweet.buy();
}