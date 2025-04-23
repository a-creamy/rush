mod lexer;
mod parser;
mod types;
use super::engine::{
    lexer::Lexer,
    parser::Parser,
    types::{Cmd, Operator},
};
use std::process::Command;

pub fn eval(input: &str) -> Result<(), String> {
    execute(Parser::new(&Lexer::new(input).lex()?).parse()?)
}

fn basic_cmd(cmd: Cmd) -> Result<(), String> {
    if let Cmd::Command(args) = cmd {
        let status = Command::new(&args[0])
            .args(&args[1..])
            .spawn()
            .and_then(|mut child| child.wait())
            .map_err(|e| format!("{}", e))?;

        if !status.success() {
            return Err(format!("Process exited with status: '{}'", status));
        }

        Ok(())
    } else {
        Ok(execute(cmd)?)
    }
}

fn execute(cmd: Cmd) -> Result<(), String> {
    match cmd {
        Cmd::Command(args) => Ok(Command::new(&args[0])
            .args(&args[1..])
            .spawn()
            .and_then(|mut child| child.wait())
            .map(|_| ())
            .map_err(|e| format!("{}", e))?),
        Cmd::BinaryOp(left, op, right) => match op {
            Operator::And => basic_cmd(*left).and_then(|_| basic_cmd(*right)),
            Operator::Or => basic_cmd(*left)
                .or_else(|_| basic_cmd(*right))
                .map_err(|e| format!("{}", e)),
            Operator::Pipe => Ok(()),
        },
    }
}
