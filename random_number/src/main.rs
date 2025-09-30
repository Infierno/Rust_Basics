use rand::prelude::*;

fn main() {
    let mut rng: ThreadRng = rand::rng();

    // Try printing a random unicode code point (probably a bad idea)!
    println!("char: {}", rng.random::<char>());

    // Try printing a random alphanumeric value instead!
    println!("Alpha: {}", rng.sample(rand::distr::Alphanumeric) as char);

    // Generate and shuffle a sequence:

    let mut nums: Vec<i32> = (1..100).collect();
    nums.shuffle(&mut rng);

    // And take a random pick (yes, we didn't need to shuffle first!):

    let a = nums.choose(&mut rng);

    println!("Vector: {:?}", a); //El println no estaba en la documentacion oficial
    //del Crate buscar alternativas para sacar el imput.

}
