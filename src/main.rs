
mod algorithms;
mod input;
mod linear;
mod parser;
mod types;
mod regex;

use input::*;
use types::Algorithm;
use parser::parse_problem;

fn main() {

    let algo = parse_problem(prueba);

    dbg!(algo.clone());

    match algo {
        Algorithm::Graphic(mut graphic) => graphic.solve(),
        Algorithm::Simplex(mut simplex) => simplex.solve(),
    }
}
