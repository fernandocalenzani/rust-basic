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

# Strings

1. Immutable Strings (&str)
   Definition: &str represents a slice of a UTF-8 encoded character sequence.
   Usage: Useful for referencing existing text data without ownership.
   Example:

```rust
let greeting = String::new(); // Declaration of an immutable string
greeting = "Hello, world!"

println!("{}", greeting); // Output: Hello, world!
```

2. Mutable Strings (String)
   Definition: String is a collection that stores a UTF-8 encoded character sequence allocated on the heap, allowing dynamic manipulation.
   Usage: Suitable for text manipulation operations that require data modifications.

Example:

```rust
let mut message = String::from("Hello"); // Creating a mutable string
message.push_str(", world!"); // Appending a string
```

3. Conversion between &str and String
   Usage: Rust provides methods for converting between &str and String.

Example:

```rust
let hello_str: &str = "Hello"; // &str
let hello_string: String = hello_str.to_string(); // Converting &str to String
let hello_again: &str = &hello_string; // Converting String to &str
```

4. Push

```rust
let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
```

5. as a Type

```rust
fn add(self, s: &str) -> String {}
```

6. to Bytes

```rust
"STRING".bytes()
```

# Hash Maps (Objects)

## Declaring

- HashMap<K, V> or HashMap<"Key", "Value">

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

## Accessing Values in a Hash Map

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);
```

## Adding a Key and Value Only If a Key Isn’t Present

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);

```
