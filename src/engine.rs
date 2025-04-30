pub mod error;
mod lexer;
mod parser;
mod types;
use crate::engine::{
    error::ShellError,
    lexer::Lexer,
    parser::Parser,
    types::{Cmd, Operator},
};
use std::io::ErrorKind;
use std::process::Command;
use std::{io, process::Stdio};

pub fn eval(input: &str) -> Result<(), ShellError> {
    execute(Parser::new(&Lexer::new(input).lex()?).parse()?)
}

fn basic_cmd(cmd: Cmd) -> Result<(), ShellError> {
    if let Cmd::Command(args) = cmd {
        let status = Command::new(&args[0])
            .args(&args[1..])
            .spawn()
            .and_then(|mut child| child.wait())
            .map_err(|e| {
                if e.kind() == ErrorKind::NotFound {
                    ShellError::CommandNotFound(args[0].clone())
                } else {
                    ShellError::from(e)
                }
            })?;

        if !status.success() {
            return Err(ShellError::CommandFailure(args[0].clone(), status));
        }

        Ok(())
    } else {
        Ok(execute(cmd)?)
    }
}

fn extract_cmd(cmd: Cmd) -> Result<Option<Vec<String>>, ShellError> {
    if let Cmd::Command(args) = cmd {
        return Ok(Some(args));
    }

    Ok(None)
}

fn execute(cmd: Cmd) -> Result<(), ShellError> {
    match cmd {
        Cmd::Command(args) => Ok(Command::new(&args[0])
            .args(&args[1..])
            .spawn()
            .and_then(|mut child| child.wait())
            .map(|_| ())
            .map_err(|e| {
                if e.kind() == ErrorKind::NotFound {
                    ShellError::CommandNotFound(args[0].clone())
                } else {
                    ShellError::from(e)
                }
            })?),
        Cmd::BinaryOp(left, op, right) => match op {
            Operator::And => basic_cmd(*left).and_then(|_| basic_cmd(*right)),
            Operator::Or => basic_cmd(*left)
                .or_else(|_| basic_cmd(*right))
                .map_err(|e| ShellError::from(e)),
            Operator::Pipe => {
                let lcmd = *left;
                let largs = match extract_cmd(lcmd.clone()) {
                    Ok(Some(args)) => args,
                    Ok(None) => {
                        return Ok(execute(lcmd.clone())?);
                    }
                    Err(_) => unreachable!(),
                };

                let rcmd = *right;
                let rargs = match extract_cmd(rcmd.clone()) {
                    Ok(Some(args)) => args,
                    Ok(None) => {
                        return Ok(execute(rcmd.clone())?);
                    }
                    Err(_) => unreachable!(),
                };

                let output = Command::new(&largs[0])
                    .args(&largs[1..])
                    .stdout(Stdio::piped())
                    .spawn()?
                    .stdout
                    .take()
                    .ok_or_else(|| {
                        ShellError::from("Failed to capture stdout from left command".to_string())
                    })?;

                Ok(Ok(Command::new(&rargs[0])
                    .args(&rargs[1..])
                    .stdin(Stdio::from(output))
                    .spawn()
                    .and_then(|mut child| child.wait())?)
                .map(|_| ())
                .map_err(|e: io::Error| {
                    if e.kind() == ErrorKind::NotFound {
                        ShellError::CommandNotFound(rargs[0].clone())
                    } else {
                        ShellError::from(e)
                    }
                })?)
            }
        },
    }
}
