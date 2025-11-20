mod control_flow;
mod conversion;
mod ownership;

use crate::control_flow::{cf, lp};
use crate::conversion::{convert_from, convert_into, convert_string, convert_try};
use crate::ownership::{first_word, ownership};

fn main() {
    cf();
    lp();
    ownership();
    println!("{}", first_word("Hello World"));

    convert_from();
    convert_into();
    convert_try();
    convert_string();

    // Pattern Matching
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background!");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as background color!");
        } else {
            println!("Using orange as background color!");
        }
    } else {
        println!("Using blue as background color!");
    }
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    });
    while let Ok(value) = rx.recv() {
        println!("{value}");
    }
}
