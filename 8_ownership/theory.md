Ownership in Rust

1. Each value in Rust has a variable that’s called its owner.
   When you allocate memory to store a value in Rust, that value is always associated with a variable that is its owner. The owner is responsible for freeing the memory associated with the value when the variable goes out of scope.

Example:

```rust
fn main() {
    let s = String::from("hello");
    // Here, 's' is the owner of the String "hello"
    // When 's' goes out of scope, the String will be automatically freed from memory
}
```

2. There can only be one owner for a value at a time.
   This means that you cannot have multiple variables that own the same value at the same time. This ensures that only one part of your code can modify the value, avoiding data races and other concurrency bugs.

Example:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is "moved" to s2
    // Now, s2 is the sole owner of the String "hello", s1 is no longer valid
    // Trying to use s1 here will cause a compile-time error
}
```

3. When the owner goes out of scope, the value will be dropped.
   When a variable that is the owner of a value goes out of scope, the value associated with that variable will be automatically freed from memory. This is done by calling the drop method.

Example:

```rust
fn main() {
    {
        let s = String::from("hello");
        // Here, 's' is the owner of the String "hello"
        // When 's' goes out of this scope, the String will be automatically freed from memory
    } // 's' goes out of scope here
}
```

# Examples:

- Normal Ownership

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5]; // Criando um vetor
    let resultado = calcular_soma(v); // Chamando a função calcular_soma

    println!("A soma dos elementos é: {}", resultado);
}

fn calcular_soma(vetor: Vec<i32>) -> i32 {
    let mut soma = 0;

    for valor in vetor { // Loop sobre os valores no vetor
        soma += valor; // Adicionando cada valor à soma
    }

    soma // Retorna a soma
}

```

- Borrowing Ownership

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5]; // Criando um vetor
    let resultado = calcular_soma(&v); // Chamando a função calcular_soma com uma referência

    println!("A soma dos elementos é: {}", resultado);
}

fn calcular_soma(vetor: &Vec<i32>) -> i32 { // Recebendo uma referência para o vetor
    let mut soma = 0;

    for valor in vetor.iter() { // Iterando sobre os valores no vetor referenciado
        soma += *valor; // Adicionando cada valor à soma
    }

    soma // Retornando a soma
}

```

- Borrowing Mutable Ownership

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5]; // Criando um vetor mutável
    modificar_vetor(&mut v); // Chamando a função modificar_vetor com uma referência mutável

    println!("O vetor modificado é: {:?}", v);
}

fn modificar_vetor(vetor: &mut Vec<i32>) {
    for valor in vetor.iter_mut() { // Iterando sobre os valores no vetor referenciado de forma mutável
        *valor *= 2; // Multiplicando cada valor por 2
    }
}

```

- Lifetime annotations

```rust
fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    let resultado = calcular_soma(&v1, &v2);
    println!("A soma dos elementos é: {}", resultado);
}

fn calcular_soma<'a, 'b>(vetor1: &'a Vec<i32>, vetor2: &'b Vec<i32>) -> i32 {
    let soma1: i32 = vetor1.iter().sum(); // Soma dos elementos do primeiro vetor
    let soma2: i32 = vetor2.iter().sum(); // Soma dos elementos do segundo vetor

    soma1 + soma2 // Retornando a soma total
}

```
