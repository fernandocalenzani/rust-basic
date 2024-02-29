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

#
