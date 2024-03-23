# Loops

In Rust, you can create loops using loop, while, and for constructs to iterate over collections or execute code repeatedly. Here's how you can use them:

1. loop:
   The loop keyword creates an infinite loop that continues until explicitly stopped using break:

```rust
loop {
    // Code block to be repeated indefinitely
    // Use break to exit the loop
    if condition {
        break;
    }
}

```

2. while:
   The while keyword creates a loop that continues as long as a condition is true:

```rust
while condition {
    // Code block to be repeated while condition is true
}

```

3. for:
   The for keyword is used to iterate over collections such as arrays, ranges, iterators, etc.:

```rust
fn main() {
    for num in 1..=5 {
        println!("Number: {}", num);
    }
}

fn main() {
    let arr = [1, 2, 3, 4, 5];

    for num in arr.iter() {
        println!("Number: {}", num);
    }
}

```

# Examples:

- [Example](src/main.rs)
