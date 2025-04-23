mod lexer;
mod parser;
mod types;
pub mod error;
use crate::engine::{
    lexer::Lexer,
    parser::Parser,
    types::{Cmd, Operator},
    error::ShellError,
};
use std::process::Command;

pub fn eval(input: &str) -> Result<(), ShellError> {
    execute(Parser::new(&Lexer::new(input).lex()?).parse()?)
}

fn basic_cmd(cmd: Cmd) -> Result<(), ShellError> {
    if let Cmd::Command(args) = cmd {
        let status = Command::new(&args[0])
            .args(&args[1..])
            .spawn()
            .and_then(|mut child| child.wait())
            .map_err(|e| ShellError::from(e))?;

        if !status.success() {
            return Err(ShellError::CommandFailure(args[0].clone(), status));
        }

        Ok(())
    } else {
        Ok(execute(cmd)?)
    }
}

fn execute(cmd: Cmd) -> Result<(), ShellError> {
    match cmd {
        Cmd::Command(args) => Ok(Command::new(&args[0])
            .args(&args[1..])
            .spawn()
            .and_then(|mut child| child.wait())
            .map(|_| ())
            .map_err(|e| ShellError::from(e))?),
        Cmd::BinaryOp(left, op, right) => match op {
            Operator::And => basic_cmd(*left).and_then(|_| basic_cmd(*right)),
            Operator::Or => basic_cmd(*left)
                .or_else(|_| basic_cmd(*right))
                .map_err(|e| ShellError::from(e)),
            Operator::Pipe => Ok(()),
        },
    }
}
