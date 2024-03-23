# RUST

## Introduction

Rust is a modern system programming language focused on performance, safety, and concurrency. It accomplishes these goals without having a garbage collector, making it a useful language for a number of use cases other languages aren’t good at. Its syntax is similar to C++, but Rust offers better memory safety while maintaining high performance.

## Why use Rust?

Rust is a systems programming language that aims to provide memory safety, concurrency, and performance with a focus on zero cost abstractions. It was originally created by Graydon Hoare at Mozilla Research, with contributions from Brendan Eich, the creator of JavaScript. Rust is appreciated for the solutions it provides to common programming language issues. Its emphasis on safety and speed, the support for concurrent programming, along with a robust type system are just a few reasons why developers choose Rust.

## Memory Safety and Zero-Cost Abstractions

Rust, a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. It is graced with the feature of “memory safety without garbage collection”, an attribute that makes Rust one of its kind. Memory safety is ensuring that software, while accessing the system’s memory, is not causing any leaks, or dangling pointers. In Rust, memory safety is accomplished through a system of ownership with a set of rules that the compiler checks at compile time. This system eliminates the need of garbage collection or manual memory management, hence ensuring swift execution of software along with a safer memory environment. This memory management feature in Rust even provides concurrent programming guaranteeing thread safety with options for shared and mutable state access that makes it harder to cause thread unsafety.

# Installation

Linux:

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
`source $HOME/.cargo/env`
`rustc --version`

# Hello Cargo in Rust

Rust's package manager, Cargo, plays a crucial role in managing Rust projects. Let's create a "Hello, Cargo!" program to showcase the basic usage of Cargo.

Create a new Rust project using Cargo:

```bash
cargo new hello_cargo
cd hello_cargo
cargo build
cargo run
```

```rust
// main function, the entry point of Rust programs
fn main() {
    // println! macro for printing to the console
    println!("Hello, Cargo!");
}
```
