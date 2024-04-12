use std::{error::Error, fmt::Display};


#[derive(Debug)]
pub struct MyError {
    detail: String,
}

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Custom error: {}", self.detail)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.detail
    }
}

#[test]
fn custom_error() {
    fn test() -> Result<(), MyError> {
        Err(MyError {
            detail: "error....".to_owned(),
        })
    }

    fn test_error() -> Result<(), Box<dyn Error>> {
        // ok
        match test() {
            Err(err) => println!("{:?}", err),
            _ => println!("ok")
        }

        // ok
        Err(Box::new(MyError {
            detail: "...22222..".to_owned(),
        }))
    }

    assert!(test().is_err());
    assert!(test_error().is_err());

    match test_error() {
        Err(err) => println!("{:?}", err),
        _ => println!("ok...")
    }
}
