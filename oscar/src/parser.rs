use std::{
    collections::HashMap,
    process
};

use crate::lexer::{lexer, Token, TokenType};
use crate::syntax::syntax;

#[derive(Debug, Clone)]
pub struct Constrain {
    pub coefs: HashMap<String, f64>,
    pub y: f64,
    pub sig: TokenType
}

#[derive(Debug, Clone)]
pub struct Z {
    pub vars: HashMap<String, f64>,
}


#[derive(Debug, Clone)]
pub struct Problem {
    pub obj: TokenType,
    pub constrains: Vec<Constrain>,
    pub func: Z,
    pub indexes_a: Vec<Vec<usize>>,
    pub coefs: Vec<f64>,
    pub coefs_indexes: Vec<usize>,
}


fn coefs_to_zero(coefs: &mut HashMap<String, f64>) {
    for v in coefs.values_mut() {
        *v = 0.0;
    }
}


fn search_first_token(tokens: Vec<Token>) -> usize {
    let mut i: usize = 0;

    while tokens[i].token_type == TokenType::EndLine {
        i += 1;
    }

    i
}


fn search_at_offset(offset: usize, tokens: Vec<Token>, token_type: TokenType) -> usize {
    let mut i: usize = 0;

    while tokens[i + offset].token_type != token_type {
        i += 1;
    }

    i + offset
}


pub fn parse(file_name: &str) -> Problem {

    let tokens = lexer(file_name);

    if !syntax(&tokens) {
        println!("Error de sintaxis");
        process::exit(1);
    }

    let mut front: usize = search_first_token(tokens.clone());

    let obj = tokens[front].token_type.clone();


    front = search_at_offset(front, tokens.clone(), TokenType::StartFunc);

    //let mut end_sentence = search_at_offset(front, tokens.clone(), TokenType::EndFunc);

    let mut i = front;

    let mut vars = HashMap::<String, f64>::new();

    while tokens[i].token_type != TokenType::EndFunc {
        if tokens[i].token_type == TokenType::Number {
            let mut j = 1;
            while tokens[i-j].token_type == TokenType::EndLine {
                j += 1;
            }
            let num = match tokens[i-j].token_type {
                TokenType::Minus => -tokens[i].value.parse::<f64>().unwrap(),
                _ => tokens[i].value.parse::<f64>().unwrap()
            };
            vars.insert(tokens[i + 1].value.clone(), num);
        }
        i += 1;
    }

    let mut tmp_coefs = vars.clone();
    let func = Z { vars };

    front = search_at_offset(i, tokens.clone(), TokenType::Number);
    let mut constrains = Vec::<Constrain>::new();


    i = front;
    while tokens[i].token_type != TokenType::EndFile {
        match tokens[i].token_type {
            TokenType::Lthan  |
            TokenType::Gthan  |
            TokenType::Lequal |
            TokenType::Gequal => {
                let sig = tokens[i].token_type.clone();
                let y = tokens[i + 1].value.parse::<f64>().unwrap();
                let coefs = tmp_coefs.clone();
                constrains.push(Constrain { coefs, y, sig });
                coefs_to_zero(&mut tmp_coefs);
                i += 1;
            },
            TokenType::Number => {
                let mut j = 1;
                while tokens[i-j].token_type == TokenType::EndLine {
                    j += 1;
                }
                let num = match tokens[i-j].token_type {
                    TokenType::Minus => -tokens[i].value.parse::<f64>().unwrap(),
                    _ => tokens[i].value.parse::<f64>().unwrap()
                };
                j = 1;
                while tokens[i+j].token_type == TokenType::EndLine {
                    j += 1;
                }
                if !tmp_coefs.contains_key(&tokens[i + j].value) {
                    println!("Error: No existe la variable {}", tokens[i + j].value);
                    i += j;
                    continue;
                }
                tmp_coefs.insert(tokens[i + j].value.clone(), num);
                i += j;
            },
            _ => {}
        }
        i += 1;
    }

    let indexes_a = Vec::new();
    let coefs = Vec::new();
    let coefs_indexes = Vec::new();
    Problem { obj, constrains, func, indexes_a, coefs, coefs_indexes }
}
