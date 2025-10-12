use std::io;
use rand::Rng;

fn main() {
    println!("Adivina un Numero");
    let str_number = rand::rng().random_range(1..=100);
    println!("El numero secreto es: {str_number}");
    println!("Por favor pon tu apuesta (un numero guey...)");

    let mut str_apuesta = String::new();

    io::stdin()
        .read_line(&mut str_apuesta)
        .expect("Chafeo esta cosa... es tu culpa");

    println!("Apostaste: {str_apuesta}")
}



