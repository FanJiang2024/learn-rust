#[test]
fn owner_ship_test() {
    // owner ship rules
    // Every value has an owner in rust.
    // There can only be one owner at a time.
    // Values are automatically dropped when the owner goes out of scope.

    #[derive(Debug)]
    struct Counter {
        number: i32,
    }

    impl Counter {
        fn new(n: i32) -> Self {
            Self {
                number: n
            }
        }

        // borrow_read 
        fn get_number(self: &Self) -> i32 {
            self.number
        }

        // borrow_mut
        fn add(self: &mut Self, increment: i32) {
            self.number += increment;
        }

        fn calcate(self: &Self, increment: i32) -> i32 {
            self.number + increment
        }

        fn combine(c1: &Self, c2: &Self) -> Self {
            Self {
                number: c1.get_number() + c2.get_number(),
            }
        }

        fn combine_move(c1: Self, c2: Self) -> Self {
            Self {
                number: c1.get_number() + c2.get_number()
            }
        }

        fn drop(self: Self) {
            println!("drop {:#?}", self);
        }
    }

    impl PartialEq for Counter {
        fn eq(&self, other: &Self) -> bool {
            self.get_number() == other.get_number()
        }
    }

    let mut c1 = Counter::new(0);
    assert_eq!(0, c1.get_number());
    assert_eq!(0, c1.get_number());
    c1.add(2);
    assert_eq!(2, c1.get_number());
    assert_eq!(2, c1.get_number());
    assert_eq!(5, c1.calcate(3));
    assert_eq!(c1.get_number(), 2);

    let c2 = Counter::new(10);
    assert_eq!(Counter::combine(&c1, &c2), Counter::new(12));

    c2.drop();  // c2 moved!!!
    // println!("{:#?}", c2);

    let c2 = Counter::new(10);
    assert_eq!(Counter::new(12), Counter::combine_move(c1, c2));

    // c1、c2 moved!!!
    // println!("{:?}", c2)
}