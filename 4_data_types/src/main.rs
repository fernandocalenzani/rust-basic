use std::io;

fn main() {
    integer_data_type();
    integer_literals();
    operations();
    tuple_type();
    array_type();
    invalid_element_access();
}

fn integer_data_type() {
    // 8-bit signed integer
    let i8_example: i8 = -42;
    println!("i8 Example: {}", i8_example);

    // 8-bit unsigned integer
    let u8_example: u8 = 42;
    println!("u8 Example: {}", u8_example);

    // 16-bit signed integer
    let i16_example: i16 = -1000;
    println!("i16 Example: {}", i16_example);

    // 16-bit unsigned integer
    let u16_example: u16 = 1000;
    println!("u16 Example: {}", u16_example);

    // 32-bit signed integer
    let i32_example: i32 = -200000;
    println!("i32 Example: {}", i32_example);

    // 32-bit unsigned integer
    let u32_example: u32 = 200000;
    println!("u32 Example: {}", u32_example);

    // 64-bit signed integer
    let i64_example: i64 = -5000000000;
    println!("i64 Example: {}", i64_example);

    // 64-bit unsigned integer
    let u64_example: u64 = 5000000000;
    println!("u64 Example: {}", u64_example);

    // 128-bit signed integer
    let i128_example: i128 = -100000000000000000000000000000000;
    println!("i128 Example: {}", i128_example);

    // 128-bit unsigned integer
    let u128_example: u128 = 100000000000000000000000000000000;
    println!("u128 Example: {}", u128_example);

    // Machine architecture dependent signed integer
    let isize_example: isize = -42;
    println!("isize Example: {}", isize_example);

    // Machine architecture dependent unsigned integer
    let usize_example: usize = 42;
    println!("usize Example: {}", usize_example);
}

fn integer_literals() {
    // Decimal
    let decimal_example = 98_222;
    println!("Decimal Example: {}", decimal_example);

    // Hexadecimal
    let hex_example = 0xff;
    println!("Hex Example: {:x}", hex_example);

    // Octal
    let octal_example = 0o77;
    println!("Octal Example: {:o}", octal_example);

    // Binário
    let binary_example = 0b1111_0000;
    println!("Binary Example: {:b}", binary_example);

    // Byte (u8 only)
    let byte_example: u8 = b'A';
    println!("Byte Example: {}", byte_example as char);
}

fn operations() {
    // addition
    let sum = 5 + 10;
    println!("op: 5 + 10 = {}", sum as f32);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("op: 95.5 - 4.3 = {}", difference as f32);

    // multiplication
    let product = 4 * 30;
    println!("op:  4 * 30 = {}", product as f32);

    // division
    let quotient = 56.7 / 32.2;
    println!("op: 56.7 / 32.2 = {}", quotient as f32);

    let truncated = -5 / 3;
    println!("op: -5 / 3 = {}", truncated as f32);

    // remainder
    let remainder = 43 % 5;
    println!("op: 43 % 5 = {}", remainder as f32);

    // boolean
    let bo: bool = false;
    println!("op: boolean = {}", bo as bool);

    // char
    let bo: bool = false;
    println!("op: boolean = {}", bo as bool);

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("op: {}, {}, {}", c, z, heart_eyed_cat);
}

fn tuple_type() {
    let tup: (i32, f64, u8) = (500, 6.4, 1); // tuple
    let (x, y, z) = tup; // parse

    println!("parse: {}", z);

    let five_hundred = tup.0;
    println!("five_hundred: {}", five_hundred);

    let six_point_four = tup.1;
    println!("six_point_four: {}", six_point_four);

    let one = tup.2;
    println!("one: {}", one);
}

fn array_type() {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}

fn invalid_element_access() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
