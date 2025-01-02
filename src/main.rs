mod control_flow;
mod ownership;
mod types;
use crate::control_flow::{cf, lp};
use crate::ownership::{first_word, ownership};
use crate::types::{aliasing, casting, inference, literals};

fn main() {
    cf();
    lp();
    ownership();
    println!("{}", first_word("hello world"));
    casting();
    literals();
    inference();
    aliasing();
}
