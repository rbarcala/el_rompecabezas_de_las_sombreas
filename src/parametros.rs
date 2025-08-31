/// Rango válido para el ángulo theta del sol
const RANGO_THETA: std::ops::RangeInclusive<u32> = 10..=80;

/// Rango válido para el número de flatlanders
const RANGO_N: std::ops::RangeInclusive<u32> = 1..=100_000;

/// Estructura para almacenar los parámetros de entrada
///
/// # Campos
///
/// * `theta` - Ángulo del sol en grados
/// * `n` - Número de flatlanders a procesar
///
/// # Ejemplo
///
/// ```
/// use el_rompecabezas_de_las_sombreas::parametros::Parametros;
///
/// let parametros = Parametros {
///     theta: 45,
///     n: 100,
/// };
/// ```
pub struct Parametros {
    pub theta: u32,
    pub n: u32,
}

impl Parametros {
    /// Lee y valida los parámetros desde la primera línea de entrada.
    ///
    /// # Argumentos
    ///
    /// * `lines` - Un iterador mutable sobre las líneas de entrada
    ///
    /// # Retorna
    ///
    /// * `Ok(Parametros)` - Si los parámetros son válidos
    /// * `Err(ErrorTipo)` - Si hay errores de entrada
    ///
    /// # Errores
    ///
    /// * `ErrorTipo::LineaFaltante` - No hay línea de entrada disponible
    /// * `ErrorTipo::IO` - Error de entrada/salida al leer
    /// * `ErrorTipo::ValorFaltante` - La línea no contiene exactamente 2 valores
    /// * `ErrorTipo::NumeroInvalido` - Los valores no son números enteros válidos
    /// * `ErrorTipo::FueraDeRango` - Los valores están fuera de los rangos permitidos
    ///
    /// # Ejemplo
    ///
    /// ```no_run
    /// use std::io::{BufReader, BufRead};
    /// use el_rompecabezas_de_las_sombreas::parametros::Parametros;
    ///
    /// let input = "45 100\n";
    /// let reader = BufReader::new(input.as_bytes());
    /// let mut lineas = reader.lines();
    ///
    /// let parametros = Parametros::leer_desde_stdin(&mut lineas).unwrap();
    /// assert_eq!(parametros.theta, 45);
    /// assert_eq!(parametros.n, 100);
    /// ```
    pub fn leer_desde_stdin<R: std::io::BufRead>(
        lineas: &mut std::io::Lines<R>,
    ) -> Result<Self, crate::ErrorTipo> {
        // Leer la primera línea
        let primera_linea = match lineas.next() {
            Some(Ok(linea)) => linea,
            Some(Err(_)) => return Err(crate::ErrorTipo::IO),
            None => return Err(crate::ErrorTipo::LineaFaltante),
        };

        // Quitamos los espacios en blanco de la línea y guardamos los números en un vector de literales
        let tokens: Vec<&str> = primera_linea.split_whitespace().collect();

        // Si al parsear, no tenemos 2 tokens o "supuestos numeros" (theta y n), retornamos error
        if tokens.len() != 2 {
            return Err(crate::ErrorTipo::ValorFaltante);
        }

        // Si no se puede parsear theta (o esta fuera de rango) retorno error
        let theta: u32 = match tokens[0].trim().parse() {
            Ok(v) => {
                if !RANGO_THETA.contains(&v) {
                    return Err(crate::ErrorTipo::FueraDeRango);
                }
                v
            }
            Err(_) => return Err(crate::ErrorTipo::NumeroInvalido),
        };

        // Si no se puede parsear n (o esta fuera de rango) retorno error
        let n: u32 = match tokens[1].trim().parse() {
            Ok(v) => {
                if !RANGO_N.contains(&v) {
                    return Err(crate::ErrorTipo::FueraDeRango);
                }
                v
            }
            Err(_) => return Err(crate::ErrorTipo::NumeroInvalido),
        };

        // Retornamos los parámetros leídos
        Ok(Parametros { theta, n })
    }
}
