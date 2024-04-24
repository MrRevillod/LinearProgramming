use std::{process, fs};

#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    Number,
    Variable,
    Plus,
    Minus,
    Lthan,
    Gthan,
    Gequal,
    Lequal,
    Equal,
    EndLine,
    StartFunc,
    EndFunc,
    Max,
    Min,
    EndFile,
    Error
}

#[derive(PartialEq, Debug, Clone)]
enum State {
    Start,
    Digit,
    Dot,
    Decimal,
    Number,
    X,
    AlphaNum,
    Var,
    Sum,
    Minus,
    Lthan,
    Gthan,
    Leq,
    Geq,
    Equal,
    EndLine,
    StartFunc,
    EndFunc,
    M,
    ErrorNumeric,
    ErrorVar,
    ErrorToken
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: u32,
    pub value: String,
}


fn set_and_push_if(i: &mut usize, cond: bool, new_state: State, state: &mut State, buffer: &mut String, c: char) {
    if cond {
        *state = new_state;
        buffer.push(c);
        *i += 1;
    }
}


fn set_state_from_alphabet(new_state: State, state: &mut State, alphabet: String, c: char) {
    for j in alphabet.chars() {
        if j == c {
            *state = new_state.clone();
        }
    }
}


fn add_new_token(token_t: &TokenType, vec_tokens: &mut Vec<Token>, buffer: String, state: &mut State, line: u32) {
    *state = State::Start;
    let token = Token {
        token_type: token_t.clone(),
        line,
        value: buffer
    };

    vec_tokens.push(token);
}


fn clean_content(content: &mut String) {

    let mut i = content.len() - 1;
    while !content.chars().nth(i).unwrap().is_alphanumeric() {
        content.pop();
        i -= 1;
    }
    *content = content.to_lowercase();
}


pub fn lexer(file_name: &str) -> Vec<Token> {

    let alphabet = String::from("+-<>=z:x\nm");
    let s_alphabet = vec![
        State::Sum,
        State::Minus,
        State::Lthan,
        State::Gthan,
        State::Equal,
        State::StartFunc,
        State::EndFunc,
        State::X,
        State::EndLine,
        State::M,
    ];

    let mut file = fs::read_to_string(file_name)
        .expect("Error: Tienes que ingresar un archivo");

    clean_content(&mut file);

    let mut c;
    let mut state = State::Start;
    let mut tokens: Vec<Token> = Vec::new();
    let mut buffer = String::with_capacity(32);
    let mut line: u32 = 1;
    let mut err_line: u32 = 1;

    let mut i: usize = 0;
    while i < file.len() {
        c = file.as_bytes()[i] as char;
        match state {
            State::Sum => add_new_token(&TokenType::Plus, &mut tokens, buffer.clone(), &mut state, line),
            State::Minus => add_new_token(&TokenType::Minus, &mut tokens, buffer.clone(), &mut state, line),
            State::Equal => add_new_token(&TokenType::Equal, &mut tokens, buffer.clone(), &mut state, line),
            State::Number => add_new_token(&TokenType::Number, &mut tokens, buffer.clone(), &mut state, line),
            State::Leq => add_new_token(&TokenType::Lequal, &mut tokens, buffer.clone(), &mut state, line),
            State::Geq => add_new_token(&TokenType::Gequal, &mut tokens, buffer.clone(), &mut state, line),
            State::Var => add_new_token(&TokenType::Variable, &mut tokens, buffer.clone(), &mut state, line),
            State::StartFunc => add_new_token(&TokenType::StartFunc, &mut tokens, buffer.clone(), &mut state, line),
            State::EndFunc => add_new_token(&TokenType::EndFunc, &mut tokens, buffer.clone(), &mut state, line),
            State::Start => {
                buffer.clear();
                state = State::Start;
                buffer.push(c);
                if c.is_alphabetic() { state = State::ErrorToken; }

                for j in 0..alphabet.len() {
                    if (alphabet.as_bytes()[j] as char) == c {
                        state = s_alphabet[j].clone();
                    }
                }

                if c.is_digit(10) { state = State::Digit; }
                i += 1;
            },
            State::M => {
                state = State::ErrorToken;
                while c.is_alphabetic() {
                    buffer.push(c);
                    i += 1;
                    c = file.as_bytes()[i] as char;
                }
                match buffer.as_str() {
                    "max" => add_new_token(&TokenType::Max, &mut tokens, buffer.clone(), &mut state, line),
                    "min" => add_new_token(&TokenType::Min, &mut tokens, buffer.clone(), &mut state, line),
                    _ => {}
                }
                i += 1;
            },
            State::EndLine => {
                add_new_token(&TokenType::EndLine, &mut tokens, buffer.clone(), &mut state, line);
                line += 1;
            },
            State::Lthan => {
                if c != '=' {
                    add_new_token(&TokenType::Lthan, &mut tokens, buffer.clone(), &mut state, line);
                }
                set_and_push_if(&mut i, c == '=', State::Leq, &mut state, &mut buffer, c);
            },
            State::Gthan => {
                if c != '=' {
                    add_new_token(&TokenType::Gthan, &mut tokens, buffer.clone(), &mut state, line);
                }
                set_and_push_if(&mut i, c == '=', State::Geq, &mut state, &mut buffer, c);
            },
            State::X  => {
                state = State::ErrorVar;
                set_and_push_if(&mut i, c.is_digit(10), State::AlphaNum, &mut state, &mut buffer, c);
            },
            State::AlphaNum => {
                state = State::ErrorVar;
                if (c as u8) < 33 {
                    state = State::Var;
                }

                set_state_from_alphabet(State::Var, &mut state, alphabet.clone(), c);
                set_and_push_if(&mut i, c.is_digit(10), State::AlphaNum, &mut state, &mut buffer, c);
            },
            State::Digit => {
                state = State::ErrorNumeric;
                if (c as u8) < 33 {
                    state = State::Number;
                }
                
                set_state_from_alphabet(State::Number, &mut state, alphabet.clone(), c);
                set_and_push_if(&mut i, c.is_digit(10), State::Digit, &mut state, &mut buffer, c);
                set_and_push_if(&mut i, c == '.', State::Dot, &mut state, &mut buffer, c);
            },
            State::Dot => {
                state = State::ErrorNumeric;
                set_and_push_if(&mut i, c.is_digit(10), State::Decimal, &mut state, &mut buffer, c);
            },
            State::Decimal => {
                state = State::ErrorNumeric;

                set_state_from_alphabet(State::Number, &mut state, alphabet.clone(), c);
                set_and_push_if(&mut i, c.is_digit(10), State::Decimal, &mut state, &mut buffer, c);
            },
            _ => {
                err_line = line;
                break;
            }
        }
    }

    if state != State::Start && state == State::Digit {
        add_new_token(&TokenType::Number, &mut tokens, buffer.clone(), &mut state, line);
    }

    add_new_token(&TokenType::EndFile, &mut tokens, "".to_string(), &mut state, line);

    if state == State::ErrorVar {
        println!("Error: Variable no reconocible, linea: {}", err_line);
        process::exit(1);
    }

    if state == State::ErrorNumeric {
        println!("Error: Número no reconocible, linea: {}", err_line);
        process::exit(1);
    }

    if state == State::ErrorToken {
        println!("Error: Token no reconocible, linea: {}", err_line);
        process::exit(1);
    }

    tokens
}
