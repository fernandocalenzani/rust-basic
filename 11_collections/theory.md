# Common Collections in Rust

Rust provides several built-in collections for storing and managing data efficiently.

_1. Storing Lists of Values with Vectors_
Definition: Vectors (Vec<T>) are resizable arrays that store elements of the same type T.
Usage: Ideal for situations where you need a dynamically sized list of elements.

Example:

```rust
let mut numbers = vec![1, 2, 3, 4, 5]; // Creating a vector of integers
numbers.push(6); // Adding an element to the vector


let mut v = Vec::new();

v.push(5);

```

_2. Storing UTF-8 Encoded Text with Strings_
Definition: Strings (String) represent UTF-8 encoded text and are mutable.
Usage: Suitable for handling textual data and performing string manipulation.
Example:
rust

```rust
let mut message = String::from("Hello"); // Creating a String
message.push_str(", world!"); // Appending a string slice
```

_3. Storing Keys with Associated Values in Hash Maps_
Definition: Hash Maps (HashMap<K, V>) store key-value pairs, allowing fast retrieval based on keys.
Usage: Useful for building associations between keys and their corresponding values.

Example:

```rust
use std::collections::HashMap;

let mut scores = HashMap::new(); // Creating an empty HashMap
scores.insert(String::from("Alice"), 100); // Inserting a key-value pair
```
