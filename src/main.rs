use std::io::{self, BufRead};

mod parametros;
mod flatlanders;
mod intervalos;
mod tests;

use parametros::Parametros;
use flatlanders::leer_flatlanders_desde_stdin;
use crate::flatlanders::crear_intervalo_de_flatlanders;
use crate::intervalos::calcular_area;


fn main() {
    let stdin = io::stdin();

    //Tomamos la entrada como un iterador, en cada next leemos una linea
    let mut lines = stdin.lock().lines();

    //Leer y validar parámetros
    let params = match Parametros::leer_desde_stdin(&mut lines) {
        Some(p) => p,
        None => return,
    };

 

    //Leer y validar flatlanders
    let flatlanders = match leer_flatlanders_desde_stdin(&mut lines, &params) {
        Some(f) => f,
        None => return,
    };

   /*
    println!("Theta: {}, N: {}", params.theta, params.n);
    */

    /*
    for (i, flatlander) in flatlanders.iter().enumerate() {
        println!("Flatlander {}: X={}, H={}", i + 1, flatlander.x, flatlander.h);
    }
    */


    //Armamos los intervalos
    let mut intervals = crear_intervalo_de_flatlanders(&flatlanders);

    //Calculamos suma (de  conjuntos) de áreas
    let area = calcular_area(&mut intervals);
    print!("{:.13}", area);

}
