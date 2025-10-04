//Primer Programa , empieza la fiesta...
// aqui estamos solo haciendo espacio...
// Esta linea mete la biblioteca de Stdio
use std::io::stdin; 

//Funcion Principal
fn main() {
  
    //Pide el nombre del usuario
    println!("What is Your Name?");
    
    //declara variable mutable cadena...
    let mut your_name = String::new();

    //Leer de la entrada estandar...
   
   stdin()
   
    .read_line(&mut your_name)
    .expect("Failed to read line");

    //Saluda al usuario imprimiendo su nombre.
    println!("Hello {}" , your_name)
    
}

