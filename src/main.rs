use el_rompecabezas_de_las_sombreas::procesar_entrada;
use std::io;
const CANTIDAD_DE_DECIMALES: usize = 13;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    match procesar_entrada(reader) {
        Ok(area) => print!("{:.prec$}", area, prec = CANTIDAD_DE_DECIMALES),
        Err(error) => {
            error.imprimir_error();
            std::process::exit(1);
        }
    }
}
