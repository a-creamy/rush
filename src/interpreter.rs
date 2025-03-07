mod lexer;
mod node;
mod parser;
use super::interpreter::{lexer::Lexer, node::Ast, node::LogicType, parser::Parser};
use std::{
    io::ErrorKind,
    process::{Command, Stdio},
};

pub struct Interpreter {
    // Example environment for future cases
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
                &cmd.unwrap().code().unwrap_or(-1)
            )
            .into());
        }

        Ok(())
    }

    fn logic(
        &self,
        lhs: &Ast,
        rhs: &Ast,
        logic_type: LogicType,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match logic_type {
            LogicType::And => {
                self.execute(lhs)?;
                self.execute(rhs)?;
                Ok(())
            }
            LogicType::Or => match self.execute(lhs) {
                Ok(_) => Ok(()),
                Err(_) => self.execute(rhs),
            },
        }
    }

    fn pipe(&self, lhs: &Ast, rhs: &Ast) -> Result<(), Box<dyn std::error::Error>> {
        let (left_cmd, left_args) = if let Ast::Command(args) = lhs {
            (&args[0], &args[1..])
        } else {
            return Err("Left hand side of the pipe must be a command".into());
        };

        let (right_cmd, right_args) = if let Ast::Command(args) = rhs {
            (&args[0], &args[1..])
        } else {
            return Err("Right hand side of the pipe must be a command".into());
        };

        let mut left_process = Command::new(left_cmd)
            .args(left_args)
            .stdout(Stdio::piped())
            .spawn()?;
        let left_stdout = left_process
            .stdout
            .take()
            .ok_or("Failed to capture stdout from left command")?;
        let left_status = left_process.wait()?;

        let right = Command::new(right_cmd)
            .args(right_args)
            .stdin(Stdio::from(left_stdout))
            .spawn()?
            .wait()?;

        if !left_status.success() {
            return Err(format!(
                "'{}' failed: Exit code: {}",
                left_cmd,
                left_status.code().unwrap()
            )
            .into());
        }

        if !right.success() {
            return Err(format!(
                "'{}' failed: Exit code: {}",
                right_cmd,
                right.code().unwrap()
            )
            .into());
        }

        Ok(())
    }

    fn execute(&self, node: &Ast) -> Result<(), Box<dyn std::error::Error>> {
        match node {
            Ast::Command(args) => self.command(args.to_vec()),
            Ast::Logic(lhs, rhs, logic_type) => self.logic(lhs, rhs, logic_type.clone()),
            Ast::Pipe(lhs, rhs) => self.pipe(lhs, rhs),
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
