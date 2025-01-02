pub fn ownership() {
    println!("Welcome to ownership module");
    let s1 = String::from("Hello World");
    let len = calculate_length(&s1);
    println!("The length of '{}' is  {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// slice type
pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
