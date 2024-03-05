# Ownership

Ownership is a set of rules that govern how a Rust program manages memory

Stack:

- The stack is a memory area managed efficiently and automatically.
- It stores variables with known sizes at compile time.
- Variables on the stack are organized in a "last in, first out" (LIFO) format
- Stack allocation and deallocation operations are fast, involving only pointer movement.
- Stack variables have a well-defined lifetime, typically tied to the scope in which they are created.

Heap allocation:

- The heap is a less organized and manually managed memory area.
- It is used to store data of dynamic or unknown size at compile time.
- Memory allocation and deallocation on the heap are slower than on the stack, involving searching for a free memory block and later releasing that block.
- In Rust, heap allocation is done using types like Box, Vec, String, among others.
- Heap variables can have a longer lifetime and usually need to be deallocated explicitly using the drop function or when they go out of scope.

## Ownership Rules

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

## Memory and Allocation

- The memory must be requested from the memory allocator at runtime.
- We need a way of returning this memory to the allocator when we’re done with our String.

Note: In C++, this pattern of deallocating resources at the end of an item’s lifetime is sometimes called Resource Acquisition Is Initialization (RAII). The drop function in Rust will be familiar to you if you’ve used RAII patterns.

```
let s1 = String::from("hello");
let s2 = s1; // pointing to the s1 memory address
```

## Return Values and Scope

```
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

## Reference and Borrowing

In Rust, the concept of "ownership" plays a crucial role in managing memory and preventing common issues like memory leaks and data races. When passing variables to functions, Rust, by default, transfers ownership of the variable, meaning the function becomes the owner of the data, and the original variable cannot be used afterward.

The example function print_first_char(s: String) demonstrates ownership transfer. In this case, the function takes ownership of the String, allowing it to consume, modify, or be responsible for freeing the associated memory.

On the other hand, using references (&String) in a function signature, as in print_first_char(s: &String), allows borrowing without transferring ownership. The function can access the data temporarily without becoming the owner, ensuring that the original variable remains usable in the calling scope after the function call.

The ownership model in Rust helps prevent memory-related bugs and ensures safer and more predictable code. Choosing between ownership transfer and borrowing depends on the specific requirements of the function and whether you want to maintain ownership or allow temporary access to the data.

- Error

```
fn main() {
    let s1 = String::from("hello");

    // Ownership of s1 is transferred to the function
    take_ownership(s1);

    // The next line would result in a compilation error,
    // as s1 is no longer available in this scope
    // println!("Original string: {}", s1);
}

fn take_ownership(s: String) {
    // Function takes ownership of the String
    println!("Length of string: {}", s.len());
    // String is automatically dropped when it goes out of scope
}
```

- Correct

```
fn main() {
    let s1 = String::from("hello");

    take_ownership(s1);

    println!("Original string: {}", s1);
}

fn take_ownership(s: &String) {
    println!("Length of string: {}", s.len());
}
```

## Mutable References

Mutable references in Rust allow for temporary, exclusive access to a variable, enabling modification of its data. Unlike immutable references, mutable references permit changing the content of the referenced data within a limited scope. However, Rust's ownership system ensures that only one mutable reference to a particular piece of data exists at a time, preventing data races and ensuring memory safety. This feature contributes to Rust's ability to write concurrent and safe code.

Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value. This code that attempts to create two mutable references to s will fail

```
- Wrong
{
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}

- Correct
{
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}

```

Rust enforces a similar rule for combining mutable and immutable references. This code results in an error:

```
{
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}
```

```
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

## Dangling References

In Rust programming language, "dangling references" refer to references that point to memory that has been deallocated, meaning the memory is no longer valid. This situation can lead to undefined behavior, crashes, or other unexpected issues in a program. Rust's ownership and borrowing system is designed to prevent dangling references and ensure memory safety.

Here's a brief explanation of how Rust helps prevent dangling references:

Ownership System: Rust employs an ownership system with strict rules to manage memory. Each value in Rust has a variable that is its "owner," and there can be only one owner at a time. When the owner goes out of scope, Rust automatically deallocates the memory associated with the value.

Borrowing and References: Rather than transferring ownership, Rust allows borrowing and referencing values. Borrowing allows a function or code block to temporarily access a value without taking ownership. References ensure that the borrowed value lives only as long as the reference.

Lifetime Annotations: Rust uses lifetime annotations to track the scope for which references are valid. This helps prevent references from outliving the data they point to.

```
- wrong way
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}


- right way
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> String {
    let s = String::from("hello");

    s
}

```

## The Slice Type

In Rust, a slice is a data type that represents a view into a contiguous sequence of elements. Slices are a flexible and efficient way to work with portions of arrays, vectors, and other data structures. They do not own the data; instead, they provide a borrowed reference to a section of the original data.

The syntax for a slice in Rust is &[T], where T is the type of the elements in the slice. Slices are used to reference a range of elements within a collection. The two main types of slices are:

Immutable Slices (&[T]):

Represented by a reference to an array or a part of a vector.
Allow read-only access to the elements.
Mutable Slices (&mut [T]):

Represented by a mutable reference to an array or a part of a vector.
Allow both read and write access to the elements.

```
fn main() {
    let original_array = [1, 2, 3, 4, 5];

    // Creating an immutable slice
    let slice: &[i32] = &original_array[1..4]; // Slice of elements from index 1 to 3 (inclusive)

    println!("Immutable Slice: {:?}", slice);

    // Creating a mutable slice
    let mut mutable_slice: &mut [i32] = &mut original_array[1..4]; // Mutable slice of elements from index 1 to 3
    mutable_slice[0] = 10; // Modifying an element in the mutable slice

    println!("Modified Mutable Slice: {:?}", mutable_slice);
}

```
