fn main() {
    by_moving();
    by_cloning();
    by_mutating();
}

fn by_moving() {
    let hello: String = "hello ".to_string();
    let world: &'static str = "world!"; //&'static str 
    //referencia estatica que no cambia por todo el programa.
    // Mover hello a una nueva variable
    let hello_world: String = hello + world;
    // hello no puede usarse mas...
    println!("{}", hello_world); //imprime hello_world 
}

fn by_cloning() {
    let hello: String = "hello ".to_string();
    let world: &'static str = "world!";

    // Creando copia de hello y moviendolo a una nueva variable
    let hello_world: String = hello.clone() + world;
    // hello puede seguir usandose...
    println!("{}", hello_world);
}

fn by_mutating() {
    let mut hello: String = "hello ".to_string(); // espacio despues de hello
    let world: &'static str = "world!";

    // hello es modificada en el lugar
    hello.push_str(world);
    // hello es usable y modificable
    println!("{}", hello); //imprime contenido de hello
}
