

//Implemento estos traits, sin embargo no uso los metodos .clone() ni .copy()
//Esto lo hago ya que no necesito duplicar intervalos, solo necesito acceder a ellos
//mediante sus referencias
#[derive(Clone, Copy)]

pub struct Interval {
    pub start: f64,
    pub end: f64,
}


///Sumamos las áreas de los intervalos (si no se superponen)
/// 
///Si se superponen, prolongamos el intervalo actual
pub fn calcular_area(intervalos: &mut [Interval]) -> f64 {

    
    let mut area = 0.0;
    let mut current: Option<Interval> = None;

    //Ordenamiento inplace
    intervalos.sort_by(|a, b| a.start.total_cmp(&b.start));


    //Se desreferencia, asi que implemento el trait Copy
    for interval in intervalos.iter() {
        if let Some(mut c) = current {
            if interval.start <= c.end {
                c.end = c.end.max(interval.end);
                current = Some(c);
            } else {
                area += c.end - c.start;
                current = Some(*interval);
            }
        } else {
            current = Some(*interval);
        }
    }

    if let Some(c) = current {
        area += c.end - c.start;
    }

    area
}