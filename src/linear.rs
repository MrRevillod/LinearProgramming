
#![allow(dead_code)]

use crate::types::*;

pub fn redondear_a_decimales(numero: f64, decimales: usize) -> f64 {
    let factor = 10.0_f64.powi(decimales as i32);
    (numero * factor).round() / factor
}

pub fn truncar_a_decimales(numero: f64, decimales: usize) -> f64 {
    let factor = 10.0_f64.powi(decimales as i32);
    (numero * factor).trunc() / factor
}

pub fn determinant(a: &A) -> f64 {
    (a[0][0] * a[1][1] - (a[1][0] * a[0][1])) as f64
}

pub fn casi_cero(num: &mut f64) -> bool {
    *num = truncar_a_decimales(*num, 3);
    *num = redondear_a_decimales(*num, 2);
    return *num < 0f64
}

pub fn cramer(a: &A, b: &B) -> Result<Point, &'static str> {
    
    let det_a = determinant(a);

    if det_a == 0.0 as f64 || det_a.abs() <= f64::EPSILON {
        return Err("Determinant error");
    }

    let x = {

        let mut temp_a = a.clone();

        for i in 0..a.len() {
            temp_a[i][0] = b[i];
        }

        determinant(&temp_a) / det_a
    };

    let y = {

        let mut temp_a = a.clone();

        for i in 0..a.len() {
            temp_a[i][1] = b[i];
        }

        determinant(&temp_a) / det_a
    };

    Ok(Point { x, y })
}

