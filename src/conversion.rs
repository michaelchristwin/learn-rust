use std::convert::{From, Into, TryFrom, TryInto};
use std::fmt;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

pub fn convert_from() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
}

impl Into<i32> for Number {
    fn into(self) -> i32 {
        self.value
    }
}

pub fn convert_into() {
    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

pub fn convert_try() {
    //TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    //TryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of rsdius {}", self.radius)
    }
}

pub fn convert_string() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
}
