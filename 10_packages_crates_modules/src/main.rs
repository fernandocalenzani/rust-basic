use libs;

mod modules {
    pub mod math;
}

use modules::math::operations as math;

fn main() {
    // Agora você pode chamar crave::math::add() de forma abreviada
    let result1 = math::add(5, 3);
    let result2 = math::sub(1, 1);
    let result3 = math::mult(1, 1);
    let result4 = math::div(1, 1);

    println!("Resultado: {}", result1);
    println!("Resultado: {}", result2);
    println!("Resultado: {}", result3);
    println!("Resultado: {}", result4);

}
