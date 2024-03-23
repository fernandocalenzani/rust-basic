fn main() {
    println!("Looping...");
    common_loop();

    println!("while_loop...");
    while_loop();

    println!("for_loop...");
    for_loop();

    println!("forin_loop...");
    forin_loop();
}

fn common_loop() {
    let mut count = 0;

    loop {
        println!("Count: {}", count);
        count += 1;

        if count >= 5 {
            break;
        }
    }
}

fn while_loop() {
    let mut count = 0;

    while count < 5 {
        println!("Count: {}", count);
        count += 1;
    }
}

fn for_loop() {
    for num in 1..=5 {
        println!("Number: {}", num);
    }
}

fn forin_loop() {
    let arr = [1, 2, 3, 4, 5];

    for num in arr.iter() {
        println!("Number: {}", num);
    }
}
