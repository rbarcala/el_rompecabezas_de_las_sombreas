use std::io::BufRead;

const RANGO_THETA: std::ops::RangeInclusive<u32> = 10..=80;
const RANGO_N: std::ops::RangeInclusive<usize> = 1..=100_000;

/// Estructura para almacenar los parámetros de entrada
pub struct Parametros {
    pub theta: u32,
    pub n: usize,
}

impl Parametros {
    /// Lee y valida los parámetros desde la primera línea de entrada
    pub fn leer_desde_stdin(lines: &mut std::io::Lines<std::io::StdinLock<'_>>) -> Option<Self> {
        // Leer la primera línea
        let primera_linea = match lines.next() {
            Some(Ok(line)) => line,
            Some(Err(_)) => {
                eprintln!("Error: no se pudo leer la primera línea");
                return None;
            }
            None => {
                eprintln!("Error: no hay entrada");
                return None;
            }
        };

        // Procesar la primera línea
        let tokens: Vec<&str> = primera_linea.split_whitespace().collect();

        if tokens.len() != 2 {
            eprintln!("Error: se esperaban dos valores en la primera línea");
            return None;
        }

        let theta: u32 = match tokens[0].trim().parse().ok() {
            Some(v) => {
                if !RANGO_THETA.contains(&v) {
                    eprintln!("Error: theta fuera de rango");
                    return None;
                }
                v
            }
            None => {
                eprintln!("Error: se esperaba un número para theta");
                return None;
            }
        };

        let n: usize = match tokens[1].trim().parse().ok() {
            Some(v) => {
                if !RANGO_N.contains(&v) {
                    eprintln!("Error: n fuera de rango");
                    return None;
                }
                v
            }
            None => {
                eprintln!("Error: se esperaba un número para n");
                return None;
            }
        };

        Some(Parametros { theta, n })
    }
}