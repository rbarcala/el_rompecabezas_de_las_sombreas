use crate::intervalos::Intervalo;
use crate::parametros::Parametros;

//Rango válido para la posición X
const RANGO_X: std::ops::RangeInclusive<u32> = 0..=300_000;

/// Rango válido para la altura H
const RANGO_H: std::ops::RangeInclusive<u32> = 1..=1000;

/// Representa un flatlander (ser plano) con posición y longitud de sombra calculada.
///
/// Un flatlander proyecta una sombra cuya longitud se calcula usando trigonometría
/// basada en su altura y el ángulo de incidencia de la luz.
///
/// # Campos
///
/// * `x` - La posición del flatlander en el eje X
/// * `l` - La longitud de la sombra
///
/// # Ejemplo
///
/// ```
/// use el_rompecabezas_de_las_sombreas::flatlanders::Flatlander;
///
/// let flatlander = Flatlander::new(10, 20, 45);
/// assert_eq!(flatlander.x, 10);
/// // Para theta=45°, L = H/tan(45°) = 20/1 = 20
/// assert!((flatlander.l - 20.0).abs() < 0.001);
/// ```
///
///
#[derive(Debug, PartialEq)]
pub struct Flatlander {
    pub x: u32,
    pub l: f64,
}

/// Implementación de métodos para Flatlander
impl Flatlander {
    /// Crea un nuevo flatlander calculando automáticamente la longitud de su sombra.
    ///
    /// La longitud de la sombra se calcula usando la fórmula trigonométrica:
    /// L = H / tan(theta)
    ///
    /// # Argumentos
    ///
    /// * `x` - La posición del flatlander en el eje X
    /// * `h` - La altura del flatlander
    /// * `theta` - El ángulo de incidencia de la luz en grados
    ///
    /// # Retorna
    ///
    /// Un nuevo `Flatlander` con la posición especificada y la longitud de sombra calculada
    ///
    /// # Ejemplo
    ///
    /// ```
    /// use el_rompecabezas_de_las_sombreas::flatlanders::Flatlander;
    ///
    /// let flatlander = Flatlander::new(5, 10, 45);
    /// // Para 45°, tan(45°) = 1, entonces L = 10/1 = 10
    /// assert!((flatlander.l - 10.0).abs() < 0.001);
    /// ```
    pub fn new(x: u32, h: u32, theta: u32) -> Self {
        // L = H / tan(theta)
        let l = h as f64 / (std::f64::consts::PI / 180.0 * theta as f64).tan();

        Self { x, l }
    }
}

/// Esta función lee exactamente `n` líneas de entrada, donde cada línea debe contener
/// la posición X y la altura H del flatlander.
/// Valida valores con los rangos permitidos.
///
/// # Argumentos
///
/// * `lineas` - Un iterador mutable sobre las líneas de entrada
/// * `params` - Los parámetros que contienen el número de flatlanders y el ángulo theta
///
/// # Retorna
///
/// `Result<Vec<Flatlander>, ErrorTipo>` - Un vector de flatlanders si todo es válido,
/// o un error específico si hay problemas de formato, valores fuera de rango, etc.
///
/// # Errores
///
/// * `ErrorTipo::IO` - Error al leer una línea
/// * `ErrorTipo::LineaFaltante` - No hay suficientes líneas de entrada
/// * `ErrorTipo::ValorFaltante` - Una línea no tiene exactamente 2 valores
/// * `ErrorTipo::NumeroInvalido` - Un valor no se puede parsear como número
/// * `ErrorTipo::FueraDeRango` - Un valor está fuera del rango permitido
pub fn leer_flatlanders_desde_stdin<R: std::io::BufRead>(
    lineas: &mut std::io::Lines<R>,
    params: &Parametros,
) -> Result<Vec<Flatlander>, crate::ErrorTipo> {
    let mut flatlanders: Vec<Flatlander> = Vec::with_capacity(params.n as usize);

    //Iteramos el ciclo descripto abajo por las "n" líneas del parametro
    for _ in 0..params.n {
        // Leemos la línea correspondiente
        let linea = match lineas.next() {
            Some(Ok(line)) => line,
            Some(Err(_)) => return Err(crate::ErrorTipo::IO),
            None => return Err(crate::ErrorTipo::LineaFaltante),
        };

        // Quitamos los espacios en blanco de la línea y guardamos los números en un vector de literales
        let tokens: Vec<&str> = linea.split_whitespace().collect();

        // Si no tenemos 2 tokens o "supuestos numeros" (x y h), retornamos error
        if tokens.len() != 2 {
            return Err(crate::ErrorTipo::ValorFaltante);
        }

        // Si no se puede parsear x (o esta fuera de rango) retorno error
        let x: u32 = match tokens[0].trim().parse() {
            Ok(v) => {
                if !RANGO_X.contains(&v) {
                    return Err(crate::ErrorTipo::FueraDeRango);
                }
                v
            }
            Err(_) => return Err(crate::ErrorTipo::NumeroInvalido),
        };

        // Si no se puede parsear h (o esta fuera de rango) retorno error
        let h: u32 = match tokens[1].trim().parse() {
            Ok(v) => {
                if !RANGO_H.contains(&v) {
                    return Err(crate::ErrorTipo::FueraDeRango);
                }
                v
            }
            Err(_) => return Err(crate::ErrorTipo::NumeroInvalido),
        };

        //Guardamos en el final del vector al flatlander i
        flatlanders.push(Flatlander::new(x, h, params.theta));
    }

    //Retornamos la colección de flatlanders
    Ok(flatlanders)
}

/// Crea una lista de intervalos de sombra a partir de una lista de flatlanders.
///
/// Cada flatlander proyecta una sombra que cubre un intervalo en el eje X desde
/// su posición hasta su posición más la longitud de su sombra.
///
/// # Argumentos
///
/// * `flatlanders` - Una slice de flatlanders
///
/// # Retorna
///
/// Un `Vec<Interval>` donde cada intervalo representa la sombra de un flatlander
///
/// # Ejemplo
///
/// ```
/// use el_rompecabezas_de_las_sombreas::flatlanders::{Flatlander, crear_intervalo_de_flatlanders};
///
/// let flatlanders = vec![
///     Flatlander::new(0, 10, 45),  // Sombra de [0, 10]
///     Flatlander::new(5, 10, 45),  // Sombra de [5, 15]
/// ];
///
/// let intervalos = crear_intervalo_de_flatlanders(&flatlanders);
/// assert_eq!(intervalos.len(), 2);
/// ```
pub fn crear_intervalo_de_flatlanders(flatlanders: &[Flatlander]) -> Vec<Intervalo> {
    flatlanders
        .iter()
        .map(|f| Intervalo {
            inicio: f.x as f64,
            fin: f.x as f64 + f.l,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PRECISION_IGUALDAD_FLOAT;
    use std::io::BufRead;

    fn assert_flatlander_eq(actual: &Flatlander, esperado: &Flatlander) {
        assert_eq!(actual.x, esperado.x);
        assert!((actual.l - esperado.l).abs() < PRECISION_IGUALDAD_FLOAT);
    }

    fn assert_intervalo_eq(actual: &Intervalo, esperado: &Intervalo) {
        assert!((actual.inicio - esperado.inicio).abs() < PRECISION_IGUALDAD_FLOAT);
        assert!((actual.fin - esperado.fin).abs() < PRECISION_IGUALDAD_FLOAT);
    }

    #[test]
    fn test_comparacion_de_flatlanders_iguales() {
        let flatlander = Flatlander::new(10, 20, 45);
        let esperado = Flatlander { x: 10, l: 20.0 };

        assert_flatlander_eq(&flatlander, &esperado);
    }

    #[test]
    fn test_intervalos_esperados() {
        // Ejemplo adicional del enunciado con theta=30
        let flatlanders = vec![
            Flatlander::new(0, 100, 30),
            Flatlander::new(50, 150, 30),
            Flatlander::new(100, 200, 30),
        ];

        let intervalos = crear_intervalo_de_flatlanders(&flatlanders);
        let intervalos_esperado = [
            Intervalo {
                inicio: 0.0,
                fin: 173.20508075688772,
            },
            Intervalo {
                inicio: 50.0,
                fin: 309.8076211353316,
            },
            Intervalo {
                inicio: 100.0,
                fin: 446.41016151377545,
            },
        ];
        for i in 0..3 {
            assert_intervalo_eq(&intervalos[i], &intervalos_esperado[i]);
        }
    }

    #[test]
    fn test_leer_flatlanders_desde_stdin_correcto() {
        let input = "10 20\n5 15\n";
        let reader = std::io::BufReader::new(input.as_bytes());
        let mut lineas = reader.lines();

        let params = Parametros { theta: 45, n: 2 };
        let resultado = leer_flatlanders_desde_stdin(&mut lineas, &params);

        assert!(resultado.is_ok());
        let flatlanders = resultado.unwrap();
        assert_eq!(flatlanders.len(), 2);
        assert_flatlander_eq(&flatlanders[0], &Flatlander::new(10, 20, 45));
        assert_flatlander_eq(&flatlanders[1], &Flatlander::new(5, 15, 45));
    }

    #[test]
    fn test_leer_flatlanders_stdin_valor_faltante() {
        let input = "10\n5 15\n";
        let reader = std::io::BufReader::new(input.as_bytes());
        let mut lineas = reader.lines();

        let params = Parametros { theta: 45, n: 2 };
        let resultado = leer_flatlanders_desde_stdin(&mut lineas, &params);

        assert_eq!(resultado, Err(crate::ErrorTipo::ValorFaltante));
    }

    #[test]
    fn test_leer_flatlanders_stdin_numero_invalido() {
        let input = "abc 20\n5 15\n"; // "abc" no es un número
        let reader = std::io::BufReader::new(input.as_bytes());
        let mut lineas = reader.lines();

        let params = Parametros { theta: 45, n: 2 };
        let resultado = leer_flatlanders_desde_stdin(&mut lineas, &params);

        assert_eq!(resultado, Err(crate::ErrorTipo::NumeroInvalido));
    }

    #[test]
    fn test_leer_flatlanders_stdin_linea_faltante() {
        let input = "10 20\n"; // Solo una línea, pero params.n = 2
        let reader = std::io::BufReader::new(input.as_bytes());
        let mut lineas = reader.lines();

        let params = Parametros { theta: 45, n: 2 };
        let resultado = leer_flatlanders_desde_stdin(&mut lineas, &params);

        assert_eq!(resultado, Err(crate::ErrorTipo::LineaFaltante));
    }

    #[test]
    fn test_leer_flatlanders_stdin_fuera_de_rango_x() {
        let input = "400000 20\n"; // X > 300_000
        let reader = std::io::BufReader::new(input.as_bytes());
        let mut lineas = reader.lines();

        let params = Parametros { theta: 45, n: 1 };
        let resultado = leer_flatlanders_desde_stdin(&mut lineas, &params);

        assert_eq!(resultado, Err(crate::ErrorTipo::FueraDeRango));
    }

    #[test]
    fn test_leer_flatlanders_stdin_fuera_de_rango_h() {
        let input = "100 2000\n"; // H > 1000
        let reader = std::io::BufReader::new(input.as_bytes());
        let mut lineas = reader.lines();

        let params = Parametros { theta: 45, n: 1 };
        let resultado = leer_flatlanders_desde_stdin(&mut lineas, &params);

        assert_eq!(resultado, Err(crate::ErrorTipo::FueraDeRango));
    }
}
