#[test]
fn trait_object_box_test() {
    trait Overview {
        fn overview(&self) -> String {
            String::from("Overview")
        }
    }

    #[derive(Debug)]
    struct Person {
        name: String,
    }

    impl Overview for Person {
        fn overview(&self) -> String {
            self.name.clone()
        }
    }

    // immutable reference
    fn overview_ref(obj: &impl Overview) -> String {
        obj.overview()
    }

    let tom = Person {
        name: "Tom".to_string(),
    };
    
    // move
    fn overview_box(o: Box<dyn Overview>) -> String {
        o.overview()
    }

    assert_eq!(overview_ref(&tom), "Tom".to_owned());
    assert_eq!(overview_box(Box::new(tom)), "Tom".to_owned());

}

#[test]
fn trait_object_box_test_2() {

    trait Discount {
        fn amount(&self) -> f64;
    }

    #[derive(Debug)]
    struct Price (f64);

    impl Discount for Price {
        fn amount(&self) -> f64 {
            self.0
        }
    }

    struct TenDiscount(f64);

    impl Discount for TenDiscount {
        fn amount(&self) -> f64 {
            self.0 - 10.0
        }
    }

    struct TenPercentDiscount(f64);

    impl Discount for TenPercentDiscount {
        fn amount(&self) -> f64 {
            self.0 * 0.9
        }
    }

    fn get_total_amount(items: &Vec<&dyn Discount>) -> f64 {
        items.iter().map(|item| item.amount()).sum()
    }

    let item1 = Price(100.0);
    let item2 = TenDiscount(100.0);
    let item3 = TenPercentDiscount(100.0);

    let item_refs: Vec<&dyn Discount> = vec![&item1, &item2, &item3];
    
    assert_eq!(280.0, get_total_amount(&item_refs));
    
    // move
    let items_move: Vec<Box<dyn Discount>> = vec![Box::new(item1), Box::new(item2), Box::new(item3)];
    assert_eq!(items_move.iter().map(|item| item.amount()).sum::<f64>(), 280.0);

}