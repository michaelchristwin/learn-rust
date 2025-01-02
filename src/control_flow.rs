pub fn cf() {
    let number = 3;
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false")
    }
}

pub fn lp() {
    println!("Running a loop");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("{}", counter);
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // while loop
    println!("USING A WHILE LOOP\n");
    let mut number = 10;
    while number != 0 {
        println!("{}", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");

    // for loop
    println!("USING A FOR LOOP\n");
    let a = [10, 20, 30, 40];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    println!("Trying liftoff with new method\n");
    for number in (1..5).rev() {
        println!("{}", number)
    }
    println!("LIFTOFF🚀🚀")
}
