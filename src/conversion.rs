use std::convert::{From, Into};

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

impl Into<i32> for Number {
    fn into(self) -> i32 {
        self.value
    }
}

pub fn convert_from() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
}

pub fn convert_into() {
    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
