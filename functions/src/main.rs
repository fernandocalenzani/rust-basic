fn main() {
    print_labeled_measurement(5, 'h');
    statements();
    expressions();
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}


fn statements() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
   return 5
}

fn expressions() {
    let x = five();
    println!("The value of x is: {x}");

}
