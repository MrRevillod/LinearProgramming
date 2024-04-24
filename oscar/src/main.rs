use std::{process, env};

mod lexer;
mod syntax;
mod parser;
mod simplex;

use simplex::simplex;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Error: Faltan argumentos");
        process::exit(1);
    }

    let mut problem = parser::parse(&args[1]);
    dbg!(problem.clone());

    simplex(&mut problem);
    // dbg!(problem);
}
