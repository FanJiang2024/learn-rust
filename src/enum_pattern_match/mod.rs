#[test]
fn enum_match_test() {
    #[derive(Debug, PartialEq)]
    enum Res {
        Zero,
        OneOrTwo,
        ThreeToNine,
        EvenNumber,
        Other
    }

    fn match_number(n :i32) -> Res {
        match n {
            0 => Res::Zero,
            1 | 2 => Res::OneOrTwo,
            3..=9 => Res::ThreeToNine,
            i if i % 2 == 0 => Res::EvenNumber,
            _ => Res::Other
        }
    }

    assert_eq!(match_number(3), Res::ThreeToNine);
    assert_eq!(match_number(-1), Res::Other);
    assert_eq!(match_number(98), Res::EvenNumber);

    enum BuildingLocation {
        Number(i32),
        Name(String),
        Unknown
    }

    impl BuildingLocation {
        // method
        fn print_location(&self) {
            match self {
                BuildingLocation::Number(c) => println!("build number is {c}"),
                BuildingLocation::Name(s) => println!("build name is {s}"),
                BuildingLocation::Unknown => println!("no information"),
            }
        }

        fn call_without_self() {
            println!("I can be called only through using double colon(::).");
        }
    }

    // house is a instance of BuildingLocation.
    // Instance uses dot(.) to call its methods.
    let house = BuildingLocation::Name("fffff".to_string());
    house.print_location();
    BuildingLocation::print_location(&house);

    // crate or module uses double colon(::) to call its mothods or constants.
    BuildingLocation::call_without_self();

    let house = BuildingLocation::Number(32);
    house.print_location();

    let house = BuildingLocation::Unknown;
    house.print_location();
}