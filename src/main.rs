
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
    
    match parse_problem(INPUT_V) {
        Algorithm::Graphic(mut method) => method.solve(),
        Algorithm::Simplex(mut method) => method.solve(),
    }
}
