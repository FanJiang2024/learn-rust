

#[test]
fn overload_operators() {
    use std::ops::Add;

    #[derive(Debug, PartialEq)]
    struct Point(f64, f64);

    impl Add for Point {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self(self.0 + rhs.0, self.1 + rhs.1)
        }
    }

    let p1 = Point(1.0, 9.0);
    let p2 = Point(2.0, 8.0);

    assert_eq!(p1 + p2, Point(3.0, 17.0));

    #[derive(Debug, PartialEq, Eq)]
    struct Pos<T> {
        x: T,
        y: T,
    }

    impl<T> Add for Pos<T>
    where
        T: Add<Output = T>, // 这个约束方法很赞！！！
    {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

    let p1 = Pos { x: 2.1, y: 3.2 };
    let p2 = Pos { x: 1.2, y: 9.0 };
    assert_eq!(Pos { x: 3.3, y: 12.2 }, p1 + p2);

    #[derive(Debug, PartialEq, Eq)]
    struct Str(String);

    impl Add for Str {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
           let mut left = self.0.clone();
           left.push_str(&rhs.0);
           Self(left)
        }
    }

    let p1 = Pos { x: Str("x".to_string()), y: Str("y".to_string()) };
    let p2 = Pos { x: Str("1".to_string()), y: Str("2".to_string() )};
    assert_eq!(Pos { x: Str("x1".to_string()) , y: Str("y2".to_string()) }, p1 + p2);

}
