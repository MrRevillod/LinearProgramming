
#![allow(dead_code)]

use crate::types::*;

pub fn precision_f64(x: f64, decimals: u32) -> f64 {
    if x == 0. || decimals == 0 {
        0.
    } else {

        let shift = decimals as i32 - x.abs().log10().ceil() as i32;
        let shift_factor = 10_f64.powi(shift);

        (x * shift_factor).round() / shift_factor
    }
}

pub fn determinant(a: &A) -> f64 {
    (a[0][0] * a[1][1] - (a[1][0] * a[0][1])) as f64
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

