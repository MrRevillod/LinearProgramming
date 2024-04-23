
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

    match parse_problem(INPUT_VI) {
        Algorithm::Graphic(mut graphic) => graphic.solve(),
        Algorithm::Simplex(mut simplex) => simplex.solve(),
    }
}
