
use super::*;
use crate::types::*;

pub fn parse_problem_kind(text: &str) -> ProblemKind {
    
    let rgx = Regex::new(r"(\+|\-)").unwrap();

    match rgx.find(text) {
        Some(matched) => {
            match matched.as_str() {
                "+" => ProblemKind::Maximize,
                "-" => ProblemKind::Minimize,
                _   => panic!("[x] - Invalid problem kind")
            }
        },
        None => panic!("[x] - No match found in text")
    }
}

pub fn parse_simplex_coeff(regex: &Regex, line: &str) -> Vec<f64> {
    regex.captures_iter(line)
        .map(|cap| cap[1].replace(" ", "").parse::<f64>().unwrap())
        .collect()
}