#![allow(overflowing_literals)]

pub fn casting() {
    let decimal = 65.4321_f32;
    // cannot cast implicitly
    // eg: let integer: u8 = decimal;
    let integer = decimal as u8;
    let character = integer as char;
    println!("Casting: {} -> {} -> {}", decimal, integer, character);
}

pub fn literals() {
    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literal, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f))
}
pub fn inference() {
    let elem = 5u8;
    let mut vec = Vec::new();
    vec.push(elem);
    println!("{:?}", vec);
}

type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

pub fn aliasing() {
    let nanoseconds: NanoSecond = 5 as u64;
    let inches: Inch = 2 as U64;
    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}

enum Avatar {
    Eren,
    Armin,
}

struct Character {
    person: Avatar,
    quality: String,
}

 let hero = Character{
    person: Avatar::Eren,
    quality:String::from("Decisive"),
}
