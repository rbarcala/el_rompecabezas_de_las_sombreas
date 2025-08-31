use el_rompecabezas_de_las_sombreas::{
    flatlanders::leer_flatlanders_desde_stdin, parametros::Parametros,
};
use std::io::{BufRead, BufReader, Cursor};

#[test]
fn test_parametros_validos() {
    let input = "45 3\n50 150\n0 100\n100 200\n";
    let reader = Cursor::new(input);
    let buf_reader = BufReader::new(reader);
    let mut lines = buf_reader.lines();

    match Parametros::leer_desde_stdin(&mut lines) {
        Ok(params) => {
            assert_eq!(params.theta, 45);
            assert_eq!(params.n, 3);
        }
        Err(_) => panic!("Se esperaba un resultado válido"),
    }
}

#[test]
fn test_parametros_theta_invalido() {
    let input = "5 3\n"; // theta = 5 (fuera del rango 10-80)
    let reader = Cursor::new(input);
    let buf_reader = BufReader::new(reader);
    let mut lines = buf_reader.lines();

    let resultado = Parametros::leer_desde_stdin(&mut lines);
    assert!(resultado.is_err());
}

#[test]
fn test_parametros_n_invalido() {
    let input = "45 0\n"; // n = 0 (debe ser >= 1)
    let reader = Cursor::new(input);
    let buf_reader = BufReader::new(reader);
    let mut lines = buf_reader.lines();

    let resultado = Parametros::leer_desde_stdin(&mut lines);
    assert!(resultado.is_err());
}

#[test]
fn test_leer_flatlanders_validos() {
    let input = "45 2\n10 20\n30 40\n";
    let reader = Cursor::new(input);
    let buf_reader = BufReader::new(reader);
    let mut lines = buf_reader.lines();

    // Primero leer parámetros
    let params = Parametros::leer_desde_stdin(&mut lines).expect("Parámetros válidos");

    // Luego leer flatlanders
    match leer_flatlanders_desde_stdin(&mut lines, &params) {
        Ok(flatlanders) => {
            assert_eq!(flatlanders.len(), 2);
            assert_eq!(flatlanders[0].x, 10);
            assert_eq!(flatlanders[1].x, 30);
        }
        Err(_) => panic!("Se esperaba un resultado válido"),
    }
}

#[test]
fn test_leer_flatlanders_formato_invalido() {
    let input = "45 2\n10 abc\n30 40\n"; // h = abc (no es un número)
    let reader = Cursor::new(input);
    let buf_reader = BufReader::new(reader);
    let mut lines = buf_reader.lines();

    let params = Parametros::leer_desde_stdin(&mut lines).expect("Parámetros válidos");
    let resultado = leer_flatlanders_desde_stdin(&mut lines, &params);
    assert!(resultado.is_err());
}

#[test]
fn test_leer_flatlanders_faltan_lineas() {
    let input = "45 2\n10 20\n"; // Solo 1 línea, pero n = 2
    let reader = Cursor::new(input);
    let buf_reader = BufReader::new(reader);
    let mut lines = buf_reader.lines();

    let params = Parametros::leer_desde_stdin(&mut lines).expect("Parámetros válidos");
    let resultado = leer_flatlanders_desde_stdin(&mut lines, &params);
    assert!(resultado.is_err());
}
