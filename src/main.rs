mod control_flow;
mod conversion;
mod ownership;
mod types;
use crate::control_flow::{cf, lp};
use crate::conversion::{convert_from, convert_into};
use crate::ownership::{first_word, ownership};
use crate::types::{aliasing, casting, inference, literals};

fn main() {
    cf();
    lp();
    ownership();
    println!("{}", first_word("Hello World"));
    casting();
    literals();
    inference();
    aliasing();
    convert_from();
    convert_into();
}
