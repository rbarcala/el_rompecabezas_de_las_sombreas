use el_rompecabezas_de_las_sombreas::{ErrorTipo, procesar_entrada};
use std::io::Cursor;

#[test]
fn test_ejemplo_basico_45_grados() {
    let input = "45 2\n0 10\n5 10\n";
    let reader = Cursor::new(input);

    let resultado = procesar_entrada(reader).unwrap();
    assert!((resultado - 15.0).abs() < 0.001);
}

#[test]
fn test_ejemplo_enunciado_30_grados() {
    let input = "30 3\n50 150\n0 100\n100 200\n";
    let reader = Cursor::new(input);

    let resultado = procesar_entrada(reader).unwrap();
    // Aproximadamente 446.41
    assert!((resultado - 446.4101615137755).abs() < 0.001);
}

#[test]
fn test_ejemplo_enunciado_45_grados_tres_flatlanders() {
    let input = "45 3\n50 150\n0 100\n100 200\n";
    let reader = Cursor::new(input);

    let resultado = procesar_entrada(reader).unwrap();
    // Aproximadamente 300.0
    assert!((resultado - 300.0).abs() < 0.001);
}

#[test]
fn test_un_solo_flatlander() {
    let input = "45 1\n10 20\n";
    let reader = Cursor::new(input);

    let resultado = procesar_entrada(reader).unwrap();
    assert!((resultado - 20.0).abs() < 0.001); // L = 20/tan(45°) = 20
}

#[test]
fn test_sin_superposicion() {
    let input = "45 2\n0 10\n20 10\n";
    let reader = Cursor::new(input);

    let resultado = procesar_entrada(reader).unwrap();
    assert!((resultado - 20.0).abs() < 0.001); // [0,10] + [20,30] = 10 + 10 = 20
}

#[test]
fn test_entrada_invalida_theta_fuera_rango() {
    let input = "5 1\n10 20\n"; // theta = 5 (fuera del rango 10-80)
    let reader = Cursor::new(input);

    let resultado = procesar_entrada(reader);
    assert!(resultado.is_err());
    match resultado {
        Err(ErrorTipo::FueraDeRango) => (), // Éxito
        _ => panic!("Se esperaba error FueraDeRango"),
    }
}

#[test]
fn test_entrada_invalida_formato() {
    let input = "45 abc\n10 20\n"; // N no es un número
    let reader = Cursor::new(input);

    let resultado = procesar_entrada(reader);
    assert!(resultado.is_err());
    match resultado {
        Err(ErrorTipo::NumeroInvalido) => (), // Éxito
        _ => panic!("Se esperaba error NumeroInvalido"),
    }
}

#[test]
fn test_entrada_invalida_faltan_lineas() {
    let input = "45 2\n10 20\n"; // Faltan líneas (solo hay 1 flatlander, necesita 2)
    let reader = Cursor::new(input);

    let resultado = procesar_entrada(reader);
    assert!(resultado.is_err());
    match resultado {
        Err(ErrorTipo::LineaFaltante) => (), // Éxito
        _ => panic!("Se esperaba error LineaFaltante"),
    }
}
