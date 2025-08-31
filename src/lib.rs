//! Este crate calcula el área total cubierta por las sombras
//! de varios flatlanders iluminados por el sol en un ángulo específico.
//!
//! ## Ejemplo de uso
//!
//! ```no_run
//! use std::io::BufReader;
//! use el_rompecabezas_de_las_sombreas::procesar_entrada;
//!
//! let input = "45 2\n0 10\n5 15\n";
//! let reader = BufReader::new(input.as_bytes());
//! let area = procesar_entrada(reader).unwrap();
//! println!("Área total: {}", area);
//! ```

pub const PRECISION_IGUALDAD_FLOAT: f64 = 1e-8; // Precisión para comparar floats

pub mod flatlanders;
pub mod intervalos;
pub mod parametros;

use flatlanders::{crear_intervalo_de_flatlanders, leer_flatlanders_desde_stdin};
use intervalos::calcular_area;
use parametros::Parametros;
use std::io::BufRead;

#[derive(Debug, PartialEq)]
///
/// Este enum define todos los posibles errores que pueden surgir al procesar
/// la entrada
///
/// # Variantes
///
/// * `IO` - Error de IO
/// * `FueraDeRango` -  Valor fuera de rango
/// * `ValorFaltante` - Linea x con menos valores de los necesarios
/// * `NumeroInvalido` -  Error al parsear un número
/// * `LineaFaltante` - Menos lineas de las necesarias
pub enum ErrorTipo {
    IO,
    FueraDeRango,
    ValorFaltante,
    NumeroInvalido,
    LineaFaltante,
}

impl ErrorTipo {
    /// Se imprime el mensaje de error específico para cada tipo mediante STDERR.
    ///
    /// # Mensajes de error
    ///
    /// * `IO` → "Error: \"IO\""
    /// * `FueraDeRango` → "Error: \"Fuera de rango\""
    /// * `ValorFaltante` → "Error: \"Valor faltante\""
    /// * `NumeroInvalido` → "Error: \"Numero invalido\""
    /// * `LineaFaltante` → "Error: \"Linea faltante\""
    ///
    /// # Ejemplo
    ///
    /// ```
    /// use el_rompecabezas_de_las_sombreas::ErrorTipo;
    ///
    /// let error = ErrorTipo::NumeroInvalido;
    /// error.imprimir_error(); // Imprime: Error: "Numero invalido"
    /// ```
    pub fn imprimir_error(&self) {
        let mensaje = match self {
            ErrorTipo::IO => "Error: \"IO\"\n",
            ErrorTipo::FueraDeRango => "Error: \"Fuera de rango\"\n",
            ErrorTipo::ValorFaltante => "Error: \"Valor faltante\"\n",
            ErrorTipo::NumeroInvalido => "Error: \"Numero invalido\"\n",
            ErrorTipo::LineaFaltante => "Error: \"Linea faltante\"\n",
        };
        eprintln!("{}", mensaje);
    }
}

/// Procesa la entrada completa y calcula el área total cubierta por las sombras.
///
/// # Algoritmo:
/// 1. Lee y valida los parámetros de entrada
/// 2. Procesa cada flatlander y calcula sus sombras en intervalos
/// 3. Suma el area de los intervalos (superpuestos o no) y lo retorna
///
/// # Argumentos
///
/// * `reader` - Un lector que implementa `BufRead` con los datos de entrada
///
/// # Formato de entrada esperado
///
/// ```text
/// theta n
/// x1 h1
/// x2 h2
/// ...
/// xn hn
/// ```
///
/// * `theta` es el ángulo del sol
/// * `n` es el número de flatlanders
/// * `xi hi` son la posición y altura del flatlander i
///
/// # Retorna
///
/// * `Ok(f64)` - El área total cubierta
/// * `Err(ErrorTipo)` - Si hay errores en la entrada
///
/// # Ejemplo
///
/// ```no_run
/// use std::io::BufReader;
/// use el_rompecabezas_de_las_sombreas::procesar_entrada;
///
/// let input = "45 2\n0 10\n5 15\n";
/// let reader = BufReader::new(input.as_bytes());
///
/// match procesar_entrada(reader) {
///     Ok(area) => println!("Área total: {:.6}", area),
///     Err(error) => error.imprimir_error(),
/// }
/// ```
pub fn procesar_entrada<R: BufRead>(reader: R) -> Result<f64, ErrorTipo> {
    let mut lineas = reader.lines();

    // Leer y validar parámetros (retorna error si falla)
    let params = Parametros::leer_desde_stdin(&mut lineas)?;

    //Leer y validar flatlanders (retorna error si falla)
    let flatlanders = leer_flatlanders_desde_stdin(&mut lineas, &params)?;

    //Creo intervalos de cada flatlander, con los cuales luego calcular el área total
    let mut intervalos = crear_intervalo_de_flatlanders(&flatlanders);
    let area = calcular_area(&mut intervalos);

    Ok(area)
}
