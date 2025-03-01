mod node;
mod tokenizer;
mod parser;
use super::interpreter::tokenizer::Tokenizer;
use super::interpreter::parser::Parser;

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
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize();

        if tokens.is_empty() {
            return;
        }

        let mut parser = Parser::new(&tokens);

        println!("{:?}", tokens);
        println!("{:?}", parser.parse());
    }
}
