#[cfg(test)]
mod tests {
    use super::*;
    use crate::intervalos::{Interval, calcular_area};
    use crate::flatlanders::{Flatlander, crear_intervalo_de_flatlanders};
    use crate::parametros::Parametros;

    #[test]
    fn test_flatlander_new() {
        // Test con theta = 45 grados
        let flatlander = Flatlander::new(10, 20, 45);
        
        assert_eq!(flatlander.x, 10);
        assert_eq!(flatlander.h, 20);
        // L = H / tan(45°) = 20 / 1 = 20
        assert!((flatlander.l - 20.0).abs() < 0.001);
    }

    #[test]
    fn test_flatlander_new_different_angles() {
        // Test con theta = 30 grados
        let flatlander = Flatlander::new(0, 100, 30);
        let expected_l = 100.0 / (30.0_f64.to_radians().tan());
        assert!((flatlander.l - expected_l).abs() < 0.001);

        // Test con theta = 60 grados
        let flatlander = Flatlander::new(50, 150, 60);
        let expected_l = 150.0 / (60.0_f64.to_radians().tan());
        assert!((flatlander.l - expected_l).abs() < 0.001);
    }

    #[test]
    fn test_crear_intervalo_de_flatlanders() {
        let flatlanders = vec![
            Flatlander::new(0, 10, 45),   // L = 10, intervalo [0, 10]
            Flatlander::new(5, 10, 45),   // L = 10, intervalo [5, 15]
        ];

        let intervals = crear_intervalo_de_flatlanders(&flatlanders);
        
        assert_eq!(intervals.len(), 2);
        assert_eq!(intervals[0].start, 0.0);
        assert_eq!(intervals[0].end, 10.0);
        assert_eq!(intervals[1].start, 5.0);
        assert_eq!(intervals[1].end, 15.0);
    }

    #[test]
    fn test_calcular_area_sin_superposicion() {
        let mut intervals = vec![
            Interval { start: 0.0, end: 5.0 },
            Interval { start: 10.0, end: 15.0 },
        ];

        let area = calcular_area(&mut intervals);
        assert_eq!(area, 10.0); // 5 + 5 = 10
    }

    #[test]
    fn test_calcular_area_con_superposicion() {
        let mut intervals = vec![
            Interval { start: 0.0, end: 10.0 },
            Interval { start: 5.0, end: 15.0 },
        ];

        let area = calcular_area(&mut intervals);
        assert_eq!(area, 15.0); // Unión: [0, 15] = 15
    }

    #[test]
    fn test_calcular_area_ejemplo_enunciado() {
        // Ejemplo del enunciado: theta=45, dos flatlanders
        let flatlanders = vec![
            Flatlander::new(0, 10, 45),   // L = 10, intervalo [0, 10]
            Flatlander::new(5, 10, 45),   // L = 10, intervalo [5, 15]
        ];

        let mut intervals = crear_intervalo_de_flatlanders(&flatlanders);
        let area = calcular_area(&mut intervals);
        
        assert_eq!(area, 15.0); // Como en el ejemplo del enunciado
    }

    #[test]
    fn test_calcular_area_intervalos_contenidos() {
        let mut intervals = vec![
            Interval { start: 0.0, end: 20.0 },
            Interval { start: 5.0, end: 15.0 }, // Contenido en el primero
        ];

        let area = calcular_area(&mut intervals);
        assert_eq!(area, 20.0); // Solo el intervalo más grande
    }

    #[test]
    fn test_calcular_area_multiples_intervalos() {
        let mut intervals = vec![
            Interval { start: 0.0, end: 5.0 },
            Interval { start: 3.0, end: 8.0 },   // Se superpone con el primero
            Interval { start: 10.0, end: 15.0 }, // Separado
            Interval { start: 12.0, end: 18.0 }, // Se superpone con el tercero
        ];

        let area = calcular_area(&mut intervals);
        // Uniones: [0, 8] y [10, 18] = 8 + 8 = 16
        assert_eq!(area, 16.0);
    }

    #[test]
    fn test_calcular_area_un_solo_intervalo() {
        let mut intervals = vec![
            Interval { start: 5.0, end: 15.0 },
        ];

        let area = calcular_area(&mut intervals);
        assert_eq!(area, 10.0);
    }

    #[test]
    fn test_calcular_area_vacio() {
        let mut intervals: Vec<Interval> = vec![];
        let area = calcular_area(&mut intervals);
        assert_eq!(area, 0.0);
    }

    #[test]
    fn test_ejemplo_30_grados() {
        // Ejemplo adicional del enunciado con theta=30
        let flatlanders = vec![
            Flatlander::new(50, 150, 30),
            Flatlander::new(0, 100, 30),
            Flatlander::new(100, 200, 30),
        ];

        let mut intervals = crear_intervalo_de_flatlanders(&flatlanders);
        let area = calcular_area(&mut intervals);
        
        // El resultado esperado según el enunciado es aproximadamente 446.41
        assert!((area - 446.4101615137755).abs() < 0.001);
    }

    #[test]
    fn test_ejemplo_45_grados_tres_flatlanders() {
        // Otro ejemplo del enunciado con theta=45
        let flatlanders = vec![
            Flatlander::new(50, 150, 45),
            Flatlander::new(0, 100, 45),
            Flatlander::new(100, 200, 45),
        ];

        let mut intervals = crear_intervalo_de_flatlanders(&flatlanders);
        let area = calcular_area(&mut intervals);
        
        // El resultado esperado según el enunciado es aproximadamente 300.0
        assert!((area - 300.0).abs() < 0.001);
    }
}
