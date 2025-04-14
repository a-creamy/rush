mod bic;
mod error;
mod executor;
mod lexer;
mod node;
mod parser;

use super::interpreter::{executor::Executor, lexer::Lexer, parser::Parser};

#[derive(Clone)]
pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn interpret<'a>(&self, input: &'a str) {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.lex();

        if tokens.is_empty() {
            return;
        }

        let mut parser = Parser::new(&tokens);
        let ast = match parser.parse() {
            Ok(result) => result,
            Err(e) => {
                eprintln!("rush: {e}");
                return;
            }
        };

        let executor = Executor::new();

        if let Err(e) = executor.execute(&ast) {
            eprintln!("rush: {e}");
        }
    }
}
