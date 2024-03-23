# Rust Data Types

Rust is a statically-typed language with a strong focus on memory safety and performance. This markdown document provides an overview of various data types in Rust.

## 1. Scalar Types

### 1.1 Integer Types

Rust supports a range of integer types, both signed and unsigned.

- `i8`, `i16`, `i32`, `i64`, `i128`: Signed integers.
- `u8`, `u16`, `u32`, `u64`, `u128`: Unsigned integers.
- `isize`, `usize`: Integer types that depend on the architecture.

### 1.2 Floating-Point Types

Rust has two floating-point types.

- `f32`: 32-bit floating point.
- `f64`: 64-bit floating point.

### 1.3 Booleans

- `bool`: Represents boolean values (`true` or `false`).

### 1.4 Characters

- `char`: Represents a single Unicode character.

## 2. Compound Types

### 2.1 Tuples

Tuples are ordered lists of fixed-size elements.

Example:

```rust
let my_tuple: (i32, f64, char) = (42, 3.14, 'a');
```

### 2.2 Arrays

Arrays have a fixed size and contain elements of the same type.

Example:

```rust
let my_array: [i32; 5] = [1, 2, 3, 4, 5];
```

References

## 3. Ownership and Borrowing

### 3.1 References

References allow borrowing values without taking ownership.

Example:

```rust
let x = 42;
let reference_to_x = &x;
```

### 3.1 Strings

Rust has two main string types.

String: A growable, heap-allocated string.
&str: A string slice, a reference to a sequence of UTF-8 bytes.

Example:

```rust
let my_string = String::from("Hello, Rust!");
let my_str_slice: &str = "Hello, Rust!";
```

## 4. Additional Types

### 4.1 Option

The Option enum represents either a value (Some) or no value (None).

Example:

```rust
let maybe_value: Option<i32> = Some(42);
let no_value: Option<i32> = None;

```

### 3.1 Result

The Result enum represents either success (Ok) or an error (Err).

Example:

```rust
fn divide(x: f64, y: f64) -> Result<f64, &'static str> {
    if y == 0.0 {
        Err("Cannot divide by zero.")
    } else {
        Ok(x / y)
    }
}

```

# Examples:

- [Example](src/main.rs)
