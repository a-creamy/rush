mod node;
mod tokenizer;
use super::interpreter::tokenizer::Tokenizer;

pub struct Interpreter {
    // Example enviroment for future cases
    // Should be used for example: keeping track ofvariables
    // env: Environment 
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    pub fn interpret<'a>(&self, input: &'a str) {
        let mut tokenizer = Tokenizer::new(input);
        println!("{:?}", tokenizer.tokenize());
    } 
}
