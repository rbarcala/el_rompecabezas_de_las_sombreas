//use crate::PRECISION_IGUALDAD_FLOAT;

//Implemento estos traits, sin embargo no uso los metodos .clone() ni .copy()
//Esto lo hago ya que no necesito duplicar intervalos, solo necesito acceder a ellos
//mediante sus referencias
#[derive(Clone, Copy, Debug, PartialEq)]

/// Representa un intervalo de números reales con un punto de inicio y fin.
///
/// # Campos
///
/// * `inicio` - El punto de inicio del intervalo
/// * `fin` - El punto final del intervalo
///
/// Ambos incluidos en el intervalo.
pub struct Intervalo {
    pub inicio: f64,
    pub fin: f64,
}

/// Calcula el área total cubierta mediante una suma de intervalos, fusionando los que se superponen.
///
/// # Argumentos
///
/// * `intervalos` - El slice mutable de intervalos
///
/// # Retorna
///
/// Un `f64` que representa la longitud total cubierta
///
/// # Ejemplo
///
/// ```
/// use el_rompecabezas_de_las_sombreas::intervalos::{Intervalo, calcular_area};
///
/// let mut intervalos = vec![
///     Intervalo { inicio: 0.0, fin: 10.0 },
///     Intervalo { inicio: 5.0, fin: 15.0 },
/// ];
///
/// let area = calcular_area(&mut intervalos);
///
/// assert_eq!(area, 15.0); // Los intervalos se fusionan en [0, 15]
/// ```
///
pub fn calcular_area(intervalos: &mut [Intervalo]) -> f64 {
    if intervalos.is_empty() {
        return 0.0;
    }

    // Ordenar in place
    intervalos.sort_by(|a, b| a.inicio.total_cmp(&b.inicio));

    let mut area_total = 0.0;

    //seteamos el primer intervalo
    let mut inicio_actual = intervalos[0].inicio;
    let mut fin_actual = intervalos[0].fin;

    // Usar iterador saltando el primer elemento (que ya procesamos)
    for intervalo in intervalos.iter().skip(1) {
        // Preguntamos si se chocan los intervalos
        if intervalo.inicio <= fin_actual {
            //Si se chocan, los fusionamos
            fin_actual = fin_actual.max(intervalo.fin);
        } else {
            //Sino, guardamos el area y empezamos uno nuevo
            area_total += fin_actual - inicio_actual;
            inicio_actual = intervalo.inicio;
            fin_actual = intervalo.fin;
        }
    }

    //Agregamos el ultimo intervalo
    area_total += fin_actual - inicio_actual;

    area_total
}
//
//
// TESTS UNITARIOS
//
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calcular_area_sin_superposicion() {
        let mut intervalos = vec![
            Intervalo {
                inicio: 0.0,
                fin: 5.0,
            },
            Intervalo {
                inicio: 10.0,
                fin: 15.0,
            },
        ];

        let area = calcular_area(&mut intervalos);
        assert_eq!(area, 10.0);
    }

    #[test]
    fn test_calcular_area_sin_superposicion_juntos() {
        let mut intervalos = vec![
            Intervalo {
                inicio: 0.0,
                fin: 5.0,
            },
            Intervalo {
                inicio: 5.0,
                fin: 10.0,
            },
        ];

        let area = calcular_area(&mut intervalos);
        assert_eq!(area, 10.0);
    }

    #[test]
    fn test_calcular_area_con_superposicion() {
        let mut intervalos = vec![
            Intervalo {
                inicio: 0.0,
                fin: 10.0,
            },
            Intervalo {
                inicio: 5.0,
                fin: 15.0,
            },
        ];

        let area = calcular_area(&mut intervalos);
        assert_eq!(area, 15.0);
    }

    #[test]
    fn test_calcular_area_intervalos_contenidos() {
        let mut intervalos = vec![
            Intervalo {
                inicio: 0.0,
                fin: 20.0,
            },
            Intervalo {
                inicio: 5.0,
                fin: 15.0,
            }, //Contenido en el primero
        ];

        let area = calcular_area(&mut intervalos);
        assert_eq!(area, 20.0);
    }

    #[test]
    fn test_calcular_area_multiples_intervalos() {
        let mut intervalos = vec![
            Intervalo {
                inicio: 0.0,
                fin: 5.0,
            },
            Intervalo {
                inicio: 3.0,
                fin: 8.0,
            }, //Se fusiona con el primero
            Intervalo {
                inicio: 10.0,
                fin: 15.0,
            },
            Intervalo {
                inicio: 12.0,
                fin: 18.0,
            }, //Se fusiona con el tercero
        ];

        let area = calcular_area(&mut intervalos);
        assert_eq!(area, 16.0);
    }

    #[test]
    fn test_calcular_area_un_solo_intervalo() {
        let mut intervalos = vec![Intervalo {
            inicio: 5.0,
            fin: 15.0,
        }];

        let area = calcular_area(&mut intervalos);
        assert_eq!(area, 10.0);
    }

    #[test]
    fn test_calcular_area_intervalos_iguales() {
        let mut intervalos = vec![
            Intervalo {
                inicio: 0.0,
                fin: 5.0,
            },
            Intervalo {
                inicio: 0.0,
                fin: 5.0,
            },
            Intervalo {
                inicio: 0.0,
                fin: 5.0,
            },
        ];

        let area = calcular_area(&mut intervalos);
        assert_eq!(area, 5.0);
    }

    #[test]
    fn test_calcular_area_vacio() {
        let mut intervalos: Vec<Intervalo> = vec![];
        let area = calcular_area(&mut intervalos);
        assert_eq!(area, 0.0);
    }

    #[test]
    fn test_calcular_area_un_punto() {
        let mut intervalos = vec![Intervalo {
            inicio: 5.0,
            fin: 5.0,
        }];

        let area = calcular_area(&mut intervalos);
        assert_eq!(area, 0.0);
    }

    #[test]
    fn test_calcular_area_punto_mas_intervalo() {
        let mut intervalos = vec![
            Intervalo {
                inicio: 5.0,
                fin: 5.0,
            },
            Intervalo {
                inicio: 10.0,
                fin: 15.0,
            },
        ];

        let area = calcular_area(&mut intervalos);
        assert_eq!(area, 5.0);
    }

    #[test]
    fn test_calcular_area_intervalo_negativo() {
        let mut intervalos = vec![
            Intervalo {
                inicio: -5.0,
                fin: 0.0,
            },
            Intervalo {
                inicio: -10.0,
                fin: -5.0,
            },
        ];

        let area = calcular_area(&mut intervalos);
        assert_eq!(area, 10.0);
    }

    #[test]
    fn test_calcular_area_intervalos_decimales() {
        let mut intervalos = vec![
            Intervalo {
                inicio: 0.5,
                fin: 1.5,
            },
            Intervalo {
                inicio: 1.5,
                fin: 2.5,
            },
            Intervalo {
                inicio: 2.5,
                fin: 3.5,
            },
        ];

        let area = calcular_area(&mut intervalos);
        assert_eq!(area, 3.0);
    }
}
