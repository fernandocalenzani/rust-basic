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
