mod node;
mod lexer;
mod parser;
use super::interpreter::{parser::Parser, lexer::Lexer};

pub struct Interpreter {
    // Example enviroment for future cases
    // Should be used for example: keeping track of variables
    // env: Environment
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    pub fn interpret<'a>(&self, input: &'a str) {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.lex();

        if tokens.is_empty() {
            return;
        }

        let mut parser = Parser::new(&tokens);

        println!("{:?}", tokens);
        println!("{:?}", parser.parse());
    }
}
