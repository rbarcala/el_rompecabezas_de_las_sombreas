use std::io::{self, BufRead};

mod parametros;
mod flatlanders;

use parametros::Parametros;
use flatlanders::leer_flatlanders_desde_stdin;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Leer y validar parámetros
    let params = match Parametros::leer_desde_stdin(&mut lines) {
        Some(p) => p,
        None => return,
    };

    println!("Theta: {}, N: {}", params.theta, params.n);

    // Leer y validar flatlanders
    let flatlanders = match leer_flatlanders_desde_stdin(&mut lines, params.n) {
        Some(f) => f,
        None => return,
    };

    // Mostrar los flatlanders leídos
    for (i, flatlander) in flatlanders.iter().enumerate() {
        println!("Flatlander {}: X={}, H={}", i + 1, flatlander.x, flatlander.h);
    }


/* 
    

    // Convertir theta a radianes
    let theta_rad = theta.to_radians();
    let tan_theta = theta_rad.tan();

    // Crear intervalos
    let mut intervals: Vec<Interval> = Vec::with_capacity(n);
    for _ in 0..n {
        let x: f64 = match iter.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => return Ok(()),
        };
        let h: f64 = match iter.next().and_then(|s| s.parse().ok()) {
            Some(v) => v,
            None => return Ok(()),
        };

        let l = h / tan_theta;
        intervals.push(Interval { start: x, end: x + l });
    }

    // Ordenar por inicio
    intervals.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());

    // Unir intervalos
    let mut total = 0.0;
    if let Some(mut current) = intervals.first().cloned() {
        for interval in &intervals[1..] {
            if interval.start <= current.end {
                if interval.end > current.end {
                    current.end = interval.end;
                }
            } else {
                total += current.end - current.start;
                current = *interval;
            }
        }
        total += current.end - current.start;
    }

    // Imprimir con precisión suficiente
    println!("{:.13}", total);

    Ok(())

    */ 
}
