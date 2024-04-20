
use std::str::FromStr;
use fraction::{Fraction, One};

#[allow(warnings)]

use crate::regex::*;
use crate::types::*;

pub fn parse_coeff(line: &str) -> Vec<f64> {
    
    COEFFS_REGEX.captures_iter(line)
        .map(|cap| cap[1].replace(" ", "").parse::<f64>().unwrap())
        .collect::<Vec<f64>>()
}

pub fn parse_problem(text: &str) -> Algorithm {

    let mut algorithm = String::new();
    let mut kind = ProblemKind::Maximize;

    let mut a  = A::new(); // A coeff matrix (left) Vec<Vec<f64>>
    let mut b  = B::new(); // B coeff matrix (right) Vec<f64>
    let mut z  = Z::new(); // Z coeff matrix Vec<f64>
    
    // Operators matrix Vec<Enum(Gt, Lt, Eq)>
    let mut operations = Operations::new();

    for line in text.lines() {

        if let Some(caps) = ALGORITHM_KIND_REGEX.captures(&line) {

            algorithm = caps[1].to_string();

            kind = match &caps[2] {
                "+" => ProblemKind::Maximize,
                "-" => ProblemKind::Minimize,
                _     => panic!("Invalid problem kind")
            };

            continue
        }

        if Z_FUNCTION_REGEX.captures(&line).is_some() {
            z = parse_coeff(line); continue
        }

        if let Some(caps) = RESULT_SIDE_REGEX.captures(&line) {
            
            a.push(parse_coeff(&line)); 

            let operation = match &caps[1] {
                ">=" => Operation::Gt,
                "<=" => Operation::Lt,
                _    => panic!("Invalid inequalitie operation")
            };

            let ineq_result = caps[2].parse::<f64>().unwrap();
            
            b.push(ineq_result);
            operations.push(operation);

            continue
        }
    }

    match algorithm.as_str() {

        "Simplex" => {

            b.insert(0, 0f64);

            match kind {
                ProblemKind::Maximize => z.insert(0, 1f64),
                ProblemKind::Minimize => z.insert(0, -1f64),
            }

            Algorithm::Simplex(
                SimplexMethod::new((kind, a, b, z, operations))
            )
        },

        "Graphic" => Algorithm::Graphic(
            GraphicMethod::new((kind, a, b, z, operations))
        ),

        _ => panic!("Invalid algorithm")
    }
}

pub fn parse_to_frac(str_number: &String) -> String {

    let mut value_vec = str_number.chars().collect::<Vec<char>>();
    let value = value_vec.clone()
        .into_iter()
        .collect::<String>()
        .parse::<f64>().unwrap()
    ;

    // if value_vec.len() > 7 && value < 0f64 {
    //     value_vec.truncate(6)
    // }
    //
    // if value_vec.len() > 7 && value > 0f64 {
    //     value_vec.truncate(5)
    // }

    let value_str = value_vec.clone()
        .into_iter()
        .collect::<String>()
    ;

    Fraction::from_str(&value_str).unwrap_or(Fraction::one()).to_string()
}
