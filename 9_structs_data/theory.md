# Structs in Rust:

1. Structs
   Structs, short for structures, allow you to create custom data types by grouping together multiple values of different types into a single unit.

- **Basic Syntax:** Structs are defined using the `struct` keyword followed by the struct's name and a block of field declarations.
- **Fields:** Fields hold the data within the struct and are defined with a name followed by a data type.

**Example:**

```rust
// Define a struct named 'Person'
struct Person {
    name: String,
    age: u32,
}

// Create an instance of 'Person'
let person1 = Person {
    name: String::from("Alice"),
    age: 30,
};
```

2. Enums
   Enums, short for enumerations, allow you to define a type by enumerating its possible variants. Each variant can optionally contain associated data.

```rust
// Define an enum named 'TrafficLight'
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// Use an enum variant
let light = TrafficLight::Red;
```

3. Arrays and Slices
   Arrays are a fixed-size collection of elements of the same type, while slices are a view into a contiguous sequence of elements.

```rust
// Define an array of integers
let arr = [1, 2, 3, 4, 5];

// Create a slice from the array
let slice = &arr[1..3];
```

4. Tuples
   Tuples allow you to group together a fixed number of values of different types into a single compound value.

```rust
// Define a tuple
let tuple: (i32, f64, u8) = (42, 3.14, 7);

// Access elements of the tuple
let (x, y, z) = tuple;

```

5. Collections
   Rust provides several built-in collection types such as vectors, strings, and hash maps that allow you to store and manipulate data dynamically.

```rust
// Define a vector of strings
let mut vec =  !["apple", "banana", "cherry"];

// Add an element to the vector
vec.push("date");

```

# Examples:

- [Example](src/main.rs)

