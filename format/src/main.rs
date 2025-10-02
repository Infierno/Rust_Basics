fn main () {
    let color = "rojo";
    // los "{}" en la cadena de formato son remplazados por este parametro.
    let favorito = format!("Mi color favorito es {}", color);
    println!("{}", favorito);

    // Puedes agregar multiple formatos que seran puestos unos 
    // sobre los otros.
    let hola = "hola ";
    let mundo = "mundo!";
    let hola_mundo = format!("{}{}", hola, mundo);
    println!("{}", hola_mundo); // imprime ya saben que... 
                                
    // format! puede concadenar cualquier tipo de data
    // que implemente el atributo 'trait' de 'Display'como numeros
    let numero_fav = format!("Mi numero favorito es {}", 42);
    println!("{}", numero_fav); // imprime la cadena completa.
                                
    // si incluyen siertos parametros multiple veces 
    // en una cadena puedes usar parametros pocicionales.
    let untara_cantara = format!("{0}, {0}, {0}, {1}!", "untara", "cantara");
    println!("{}", untara_cantara); // imprime algo...
                                
    // Puedes nombrar tus parametros! 
    let introduccion = format!(
        "Mi nombre es {surname}, {forename} {surname}",
        surname = "Bond",
        forename= "James"
    );
    println!("{}", introduccion) // imprime algo tonto... 
    }

