fn main () {

    let mut fahr:i32;
    let mut celsius:i32;
    let lower:i32;
    let uper:i32;
    let step:i32;

    lower = 0;      // limite inferior de la tabla.
    uper = 300;     // limite superior de la tabla.
    step = 20;      // tamaño del incremento.

    fahr = lower;

    while fahr <= uper {

        celsius = 5* (fahr-32) /9;
        print!("{}\t{}\n", fahr,celsius);
        fahr= fahr + step;
    }
}
