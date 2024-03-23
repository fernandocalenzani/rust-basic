# Structuring a Rust Library

To organize a library in Rust, follow these steps:

Define Directory Structure:

Create a main directory for your library.
Within this directory, add a src directory for the source code.
Define Modules and Functions:

Within the src directory, organize your modules into subdirectories, if necessary.
Use reexporting to simplify function calls.
Configuration of Cargo.toml File:

In the Cargo.toml file, define project information and dependencies, if any.

```rust
my_lib/
│
├── Cargo.toml
│
└── src/
    ├── math/
    │   ├── operations.rs
    │   └── mod.rs
    └── lib.rs

my_app/
│
├── Cargo.toml // insert the package name here
│
└── src/
    └── main.rs //call the package name here `use package_name`

```

# Structuring Rust Code with Modules

Modules in Rust allow you to organize code within a project. Here's a summary:

Definition and Use:

Modules are declared using the mod keyword followed by the module name.
You can nest modules within other modules to create a hierarchical structure.
Visibility:

By default, items (such as functions, structs, enums) within a module are private to that module.
You can use the pub keyword to make items public and accessible from outside the module.
Reexporting:

You can reexport items from one module to another using the pub use statement.
This allows you to provide a simpler public interface while still organizing code internally.

- Structure

```rust
src/
│
├── main.rs
│
└── modules/
    └── math/
        ├── operations.rs
        └── mod.rs
```

- ./modules/math/mod.rs

```rust
pub use self::operations::add;
pub mod operations;
```

- ./modules/math/operations.rs

```rust

// Definindo a função add
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

- main.rs

```rust
use libs;

mod modules {
    pub mod math;
}

fn main() {
    // Agora você pode chamar crave::math::add() de forma abreviada
    let result = modules::math::add(5, 3);
    let libs_result = libs::add(1, 1);

    println!("Resultado: {}", result);
    println!("Resultado: {}", libs_result);
}


```
