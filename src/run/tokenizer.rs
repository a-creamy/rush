use super::node::Token;
use crate::run::node;

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    for part in input.split_whitespace() {
        match part {
            "|" => tokens.push(Token::Pipe),
            "&&" => tokens.push(Token::And),
            arg => tokens.push(Token::Arg(arg.to_string())),
        }
    }
    return tokens;
}
