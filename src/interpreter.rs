mod lexer;
mod node;
mod parser;
use super::interpreter::{lexer::Lexer, node::Ast, node::LogicType, parser::Parser};
use std::io::ErrorKind;
use std::process::Command;

pub struct Interpreter {
    // Example enviroment for future cases
    // Should be used for example: keeping track of variables
    // env: Environment
    debug: bool,
}

impl Interpreter {
    pub fn new(debug: bool) -> Self {
        Interpreter { debug }
    }

    fn command(&self, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let cmd = Command::new(&args[0])
            .args(&args[1..])
            .spawn()
            .and_then(|mut child| child.wait());

        if let Err(e) = &cmd {
            if e.kind() == ErrorKind::NotFound {
                return Err(format!("Unknown Command: {}", &args[0]).into());
            }
        }

        if !cmd.as_ref().unwrap().success() {
            return Err(format!(
                "'{}' failed: Exit code: {}",
                &args[0],
                &cmd.unwrap().code().unwrap()
            )
            .into());
        }

        return Ok(());
    }

    fn logic(
        &self,
        lhs: &Ast,
        rhs: &Ast,
        logic_type: LogicType,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match logic_type {
            LogicType::And => {
                if let Err(e) = self.execute(lhs) {
                    return Err(e);
                }
                self.execute(rhs)?;
                return Ok(());
            }
            LogicType::Or => match self.execute(lhs) {
                Ok(_) => return Ok(()),
                Err(e) => {
                    self.execute(rhs)?;
                    return Err(e);
                }
            },
        };
    }

    fn execute(&self, node: &Ast) -> Result<(), Box<dyn std::error::Error>> {
        match node {
            Ast::Command(args) => Ok(self.command(args.to_vec())?),
            Ast::Logic(lhs, rhs, logic_type) => Ok(self.logic(lhs, rhs, logic_type.clone())?),
            _ => Err("Unsupported symbol".into()),
        }
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

        if self.debug {
            println!("Tokens: {:?}", tokens);
            println!("Ast: {:?}", ast);
        }

        if let Err(e) = self.execute(&ast) {
            eprintln!("rush: {e}");
        }
    }
}
