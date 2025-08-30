use crate::intervalos::Interval;
use crate::parametros::Parametros;

const RANGO_X: std::ops::RangeInclusive<u32> = 0..=300_000;
const RANGO_H: std::ops::RangeInclusive<u32> = 1..=1000;

pub struct Flatlander {
    pub x: u32,
    pub l: f64,
}

///Crea un nuevo flatlander
impl Flatlander {
    
    pub fn new(x: u32, h: u32, theta: u32) -> Self {
        
        // L = H / tan(theta)
        let l = h as f64 / (std::f64::consts::PI / 180.0 * theta as f64).tan();

        Self { x, l }
    }
}

/// Lee y valida una lista de flatlanders desde stdin, leyendo n lineas
/// pasadas por el parámetro
pub fn leer_flatlanders_desde_stdin(
    lines: &mut std::io::Lines<std::io::StdinLock<'_>>,
    params: &Parametros,
) -> Option<Vec<Flatlander>> {

    let mut flatlanders: Vec<Flatlander> = Vec::with_capacity(params.n);

    for i in 0..params.n {
        let linea = match lines.next() {
            Some(Ok(line)) => line,
            Some(Err(_)) => {
                eprintln!("Error: no se pudo leer la línea {}", i + 2);
                return None;
            }
            None => {
                eprintln!("Error: faltan líneas de flatlanders");
                return None;
            }
        };

        let tokens: Vec<&str> = linea.split_whitespace().collect();

        if tokens.len() != 2 {
            eprintln!("Error: se esperaban dos valores en la línea {}", i + 2);
            return None;
        }

        let x: u32 = match tokens[0].trim().parse().ok() {
            Some(v) => {
                if !RANGO_X.contains(&v) {
                    eprintln!("Error: X fuera de rango en la línea {}", i + 2);
                    return None;
                }
                v
            }
            None => {
                eprintln!("Error: se esperaba un número para X en la línea {}", i + 2);
                return None;
            }
        };

        let h: u32 = match tokens[1].trim().parse().ok() {
            Some(v) => {
                if !RANGO_H.contains(&v) {
                    eprintln!("Error: H fuera de rango en la línea {}", i + 2);
                    return None;
                }
                v
            }
            None => {
                eprintln!("Error: se esperaba un número para H en la línea {}", i + 2);
                return None;
            }
        };

        flatlanders.push(Flatlander::new(x, h, params.theta));
    }

    Some(flatlanders)
}

/// Crea una lista de intervalos de espacio, de cada flatlander
pub fn crear_intervalo_de_flatlanders(flatlanders: &[Flatlander]) -> Vec<Interval> {
    flatlanders
        .iter()
        .map(|f| Interval {
            start: f.x as f64,
            end: f.x as f64 + f.l,
        })
        .collect()
}


