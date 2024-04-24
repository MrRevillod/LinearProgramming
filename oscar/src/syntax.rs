use crate::lexer::{Token, TokenType};


fn return_if_is_there(is_there: bool, token_type: TokenType) -> TokenType {
    if is_there {
        token_type
    } else {
        TokenType::Error
    }
}

fn check_first_sentence(tokens: &Vec<Token>, front: &mut usize) -> bool {

    let mut syntax_ok = tokens.iter().any(|token| token.token_type == TokenType::EndFunc);

    let mut i = *front;
    let mut token_t = tokens[i].token_type.clone();
    i += 1;
    while tokens[i].token_type != TokenType::EndFunc && syntax_ok {
        match token_t {
            TokenType::Max | TokenType::Min => {
                let is_there = vec![TokenType::StartFunc, TokenType::EndLine]
                    .contains(&tokens[i].token_type);
                token_t = return_if_is_there(is_there, tokens[i].token_type.clone());
            },
            TokenType::Plus  | TokenType::Minus => {
                let is_there: bool = vec![TokenType::Number, TokenType::EndLine]
                    .contains(&tokens[i].token_type);

                token_t = return_if_is_there(is_there, tokens[i].token_type.clone());
            },
            TokenType::Equal => {
                let is_there: bool = vec![TokenType::Number, TokenType::Plus, TokenType::Minus, TokenType::EndLine]
                    .contains(&tokens[i].token_type);

                token_t = return_if_is_there(is_there, tokens[i].token_type.clone());
            },
            TokenType::Number => {
                let is_there: bool = vec![TokenType::Variable, TokenType::EndLine]
                    .contains(&tokens[i].token_type);
                token_t = return_if_is_there(is_there, tokens[i].token_type.clone());
            },
            TokenType::Variable => {
                let is_there: bool = vec![
                    TokenType::Plus, TokenType::Minus, TokenType::Equal, TokenType::EndLine
                ].contains(&tokens[i].token_type);

                token_t = return_if_is_there(is_there, tokens[i].token_type.clone());
            },
            TokenType::StartFunc => {
                let is_there: bool = vec![TokenType::Equal, TokenType::EndLine]
                    .contains(&tokens[i].token_type);
                token_t = return_if_is_there(is_there, tokens[i].token_type.clone());
            },
            TokenType::EndLine => {
                let is_there: bool = vec![
                    TokenType::Number, TokenType::Variable, TokenType::Plus,
                    TokenType::Minus, TokenType::Equal, TokenType::EndLine
                ].contains(&tokens[i].token_type);

                token_t = return_if_is_there(is_there, tokens[i].token_type.clone());
            },
            _ => syntax_ok = false
        }
        i += 1;
    }
    if token_t == TokenType::Plus || token_t == TokenType::Minus {
        let is_there: bool = vec![TokenType::Number, TokenType::EndLine]
            .contains(&tokens[i].token_type);

        syntax_ok = return_if_is_there(is_there, tokens[i].token_type.clone()) != TokenType::Error;
    }

    i += 1;
    *front = i;

    syntax_ok
}


pub fn syntax(tokens: &Vec<Token>) -> bool {

    let mut front = 0;
    while tokens[front].token_type == TokenType::EndLine {
        front += 1;
    }

    let mut syntax_ok = check_first_sentence(tokens, &mut front);

    let mut token_t: TokenType = tokens[front].token_type.clone();
    front += 1;

    let mut i = front;
    while i < tokens.len() && syntax_ok {
        match token_t {
            TokenType::Number => {
                let is_there: bool = vec![TokenType::Variable, TokenType::EndLine, TokenType::EndFile]
                    .contains(&tokens[i].token_type);
                token_t = return_if_is_there(is_there, tokens[i].token_type.clone());
            },
            TokenType::Variable => {
                let is_there: bool = vec![
                    TokenType::Plus, TokenType::Minus, TokenType::Lequal, TokenType::Gequal,
                    TokenType::Lthan, TokenType::Gthan, TokenType::EndLine
                ].contains(&tokens[i].token_type);

                token_t = return_if_is_there(is_there, tokens[i].token_type.clone());
            },
            TokenType::Plus   |
            TokenType::Minus  |
            TokenType::Lthan  |
            TokenType::Gthan  |
            TokenType::Lequal |
            TokenType::Gequal => {
                let is_there: bool = vec![TokenType::Number, TokenType::EndLine]
                    .contains(&tokens[i].token_type);

                token_t = return_if_is_there(is_there, tokens[i].token_type.clone());
            },
            TokenType::EndLine => {
                let is_there: bool = vec![
                    TokenType::Number, TokenType::Variable, TokenType::Plus, TokenType::Minus, TokenType::Lequal,
                    TokenType::Gequal, TokenType::Lthan, TokenType::Gthan, TokenType::EndLine
                ].contains(&tokens[i].token_type);

                token_t = return_if_is_there(is_there, tokens[i].token_type.clone());
            },
            _ => syntax_ok = false
        };
        i += 1;
    }

    syntax_ok
}
