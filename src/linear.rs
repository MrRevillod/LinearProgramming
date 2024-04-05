
use crate::types::*;
use conditional::conditional;

pub fn gen_identity(len: usize) -> A {

    let mut identity = A::new();

    for i in 0..len {

        let mut row = Vec::new();

        for j in 0..len {
            row.push(conditional!(i == j ? 1.0 : 0.0));
        }

        identity.push(row);
    }

    identity
}

pub fn determinant(a: &A) -> f64 {
    (a[0][0] * a[1][1] - (a[1][0] * a[0][1])) as f64
}

pub fn cramer(a: &A, b: &B) -> Result<Point, &'static str> {

    let det_a = determinant(a);

    if det_a == 0.0 as f64 || det_a.abs() <= f64::EPSILON {
        return Err("Determinant error")
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

    Ok( Point { x, y } )
}
