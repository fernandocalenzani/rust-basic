# GET STARTED

# Rust

The Rust programming language helps you write faster, more reliable software. High-level ergonomics and low-level control are often at odds in programming language design; Rust challenges that conflict. Through balancing powerful technical capacity and a great developer experience, Rust gives you the option to control low-level details (such as memory usage) without all the hassle traditionally associated with such control.

# Installation

Linux:

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
`source $HOME/.cargo/env`
`rustc --version`

# Hello World

```
fn main() {
    println!("Hello, world!");
}

```

```
rustc hello_world.rs && ./hello_world
```

# Hello Cargo

```
cargo new hello_cargo

cd /project

cargo build

cargo run

cargo check
```

Cargo will create a new project named 'hello_cargo'.

|- project_name/
| |-- src/
| | |-- main.rs
| |-- .gitignore
| |-- Cargo.toml

# Variables and Mutability

- for standard, the variables in Rust are immutable

```
fn main() {
    let x = 5; // immutable variable

    println!("The value of x is: {x}");
    x = 6; // error message: the variable is immutable
    println!("The value of x is: {x}");
}
```

- declaring mutable variables

```
fn main() {
    let mut x = 5; // mutable variable

    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

- constants: First, you aren’t allowed to use mut with constants. Constants aren’t just immutable by default—they’re always immutable. You declare constants using the const keyword instead of the let keyword, and the type of the value must be annotated. We’ll cover types and type annotations in the next section, “Data Types”, so don’t worry about the details right now. Just know that you must always annotate the type.

- Shadowing

```
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

Output:
The value of x in the inner scope is: 12
The value of x is: 6
```

# Data Types

- Every value in Rust is of a certain data type, which tells Rust what kind of data is being specified so it knows how to work with that data.

- Integer Types:

```
Length	Signed	Unsigned
8-bit	i8	    u8
16-bit	i16	    u16
32-bit	i32	    u32
64-bit	i64	    u64
128-bit	i128	u128
arch	isize	usize
```

- Integer Literals in Rust:

```
Number literals	Example
Decimal	        98_222
Hex	            0xff
Octal	        0o77
Binary	        0b1111_0000
Byte(u8 only)	b'A'
```

- Examples:

```
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

- Numerical operations:

```
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}
```

- Boolean: `let f: bool = false;`
- Array Type: `let a: [i32; 5] = [1, 2, 3, 4, 5]; a[1]`
- Array Touple:

```
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

- The Character Type:

```
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
```

# Functions

- Statements are instructions that perform some action and do not return a value.
- Expressions evaluate to a resultant value. Let’s look at some examples.

- Statements:

```
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

output> The value of y is: 4
```

- Expressions:

```
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}

output> The value of y is: 5

```

# Comments

```
// hello, world
```

# Control Flow

- if-else

```
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

```
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}
```

# Loops

- Looping

```
fn main() {
    loop {
        println!("again!");
    }
}
```

- Loop with break

```
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

# Ownership
