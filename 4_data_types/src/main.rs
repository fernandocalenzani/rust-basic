fn main() {
    main_i32();
    print!("{}", '\n');
    main_u32();
    print!("{}", '\n');
    main_f64();
    print!("{}", '\n');
    main_bool();
    print!("{}", '\n');
    main_char();
    print!("{}", '\n');
}

fn main_i32() {
    let x: i32 = 42;
    let y: i32 = 10;

    println!("i32: {}", x);
    println!("Soma: {}", x + y);
    println!("Subtração: {}", x - y);
    println!("Multiplicação: {}", x * y);
    println!("Divisão: {}", x / y);
    println!("Resto da divisão: {}", x % y);
}

fn main_u32() {
    let x: u32 = 42;
    let y: u32 = 10;

    println!("u32: {}", x);
    println!("Soma: {}", x + y);
    println!("Subtração: {}", x - y);
    println!("Multiplicação: {}", x * y);
    println!("Divisão: {}", x / y);
    println!("Resto da divisão: {}", x % y);
}

fn main_f64() {
    let x: f64 = 3.14;
    let y: f64 = 2.0;

    println!("f64: {}", x);
    println!("Soma: {}", x + y);
    println!("Subtração: {}", x - y);
    println!("Multiplicação: {}", x * y);
    println!("Divisão: {}", x / y);
}

fn main_bool() {
    let x: bool = true;
    let y: bool = false;

    println!("bool: {}", x);
    println!("E lógico: {}", x && y);
    println!("Ou lógico: {}", x || y);
    println!("Negação lógica: {}", !x);
}

fn main_char() {
    let x: char = 'a';
    let y: char = 'b';

    println!("char: {}", x);
    println!("Conversão para inteiro: {}", x as u32);
    println!("Soma: {}", x as u8 + y as u8);
}
