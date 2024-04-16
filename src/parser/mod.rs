
pub mod utils;

use core::f64;

use regex::Regex;
use self::utils::*;
use crate::types::*;

pub fn initialize(text: &str) {
    
    let alg_regex = Regex::new(r"(Graphic|Simplex)").unwrap();

    if let Some(matched) = alg_regex.find(text) {
        match matched.as_str() {
            // "Graphic" => parse_graphic(text).solve(),
            "Simplex" => parse_simplex(text).solve(),
            _ => panic!("Invalid algorithm"),
        }
    } else {
        panic!("Invalid algorithm");
    }
}

// pub fn parse_graphic(text: &str) -> GraphicMethod {
//     
//     let mut a = A::new();
//     let mut b = B::new();
//     let mut z = Z::new();
//
//     let mut operations = Operations::new();
//     let mut inequalities = Vec::new();
//
//     let obj_rgx = Regex::new(r"z = (-?\d+)x \+ (-?\d+)y").unwrap();
//     let ec_rgx = Regex::new(r"(-?\d+)x \+ (-?\d+)y (<=|>=|=) (-?\d+)").unwrap();
//
//     let kind = parse_problem_kind(text);
//
//     for line in text.lines() {
//         
//         if let Some(caps) = obj_rgx.captures(line) {
//             z.push(caps[1].parse::<f64>().unwrap());
//             z.push(caps[2].parse::<f64>().unwrap());
//         }
//
//         if let Some(caps) = ec_rgx.captures(line) {
//             
//             let coeff = vec![
//                 caps[1].parse::<f64>().unwrap(),
//                 caps[2].parse::<f64>().unwrap(),
//             ];
//
//             let res = caps[4].parse::<f64>().unwrap();
//
//             let kind = match &caps[3] {
//                 "<=" => Operation::Lt,
//                 ">=" => Operation::Gt,
//                 "=" => Operation::Eq,
//                 _ => panic!("Inequalities operatos must be [ <= or >= or = ]"),
//             };
//
//             inequalities.push(vec![res, coeff[0].clone(), coeff[1].clone()]);
//
//             a.push(coeff);
//             b.push(res.clone());
//             operations.push(kind);
//         }
//     }
//
//     GraphicMethod::new((kind, a, b, z, operations, inequalities))
// }

pub fn parse_simplex(text: &str) -> SimplexMethod {

    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut c = Vec::new();
    let mut op = Operations::new();

    let obj_rgx = Regex::new(r"z =").unwrap();
    let coeff_rgx = Regex::new(r"([+-]?\s*\d+)\s*x\d+").unwrap();
    let result_rgx = Regex::new(r"(<=|>=|=)\s*(-?\d+)").unwrap();

    let kind = parse_problem_kind(text);

    if let Some(line) = text.lines().skip(3).next() {
        if obj_rgx.is_match(line) {
            c = parse_simplex_coeff(&coeff_rgx, line);
        }
    }

    for line in text.lines().skip(4) {
        
        a.push(parse_simplex_coeff(&coeff_rgx, line));

        if let Some(cap) = result_rgx.captures(line) {
            
            let kind = match &cap[1] {
                "<=" => Operation::Lt,
                ">=" => Operation::Gt,
                "=" => Operation::Eq,
                _ => panic!("Inequalities operatos must be [ <= or >= or = ]"),
            };

            op.push(kind);
            b.push(cap[2].parse::<f64>().unwrap());
        }
    }

    b.insert(0, 0f64);

    match kind {
        ProblemKind::Maximize => c.insert(0, 1f64),
        ProblemKind::Minimize => c.insert(0, -1f64),
    }

    SimplexMethod::new((kind, a, b, c, op))
}
