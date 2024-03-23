fn main() {
    collections_fn();
    touples_fn();
    arrays_slices();
    enum_fn();
    structur_fn();
}

fn collections_fn() {
    // Define a vector of strings
    let mut vec = vec!["apple", "banana", "cherry"];

    // Add an element to the vector
    vec.push("date");
}

fn touples_fn() {
    // Define a tuple
    let tuple: (i32, f64, u8) = (42, 3.14, 7);

    // Access elements of the tuple
    let (x, y, z) = tuple;
}

fn arrays_slices() {
    // Define an array of integers
    let arr = [1, 2, 3, 4, 5];

    // Create a slice from the array
    let slice = &arr[1..3];
}

fn enum_fn() {
    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }

    // Use an enum variant
    let light = TrafficLight::Red;
}

fn structur_fn() {
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
}
