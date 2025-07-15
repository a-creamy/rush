use std::{
    path::PathBuf,
    process::{Child, Command, ExitStatus},
};

use crate::engine::{
    error::ShellError,
    parser::{Expr, Operator},
    stream::{FileOption, Stream, StreamFile},
};

pub mod error;
pub mod lexer;
pub mod parser;
pub mod stream;

pub enum Process {
    Child(Child),
    ExitStatus(ExitStatus),
}

pub struct Config {
    pub stdout: Stream,
    pub stderr: Stream,
    pub stdin: Stream,
}

impl Config {
    pub fn new(stdout: Stream, stderr: Stream, stdin: Stream) -> Self {
        Self {
            stdout: stdout,
            stderr: stderr,
            stdin: stdin,
        }
    }
}

pub fn execute(expr: Expr, config: Option<&Config>) -> Result<Process, ShellError> {
    match expr {
        Expr::Atomic(a) => {
            if a.is_empty() {
                return Err(ShellError::Unnecassary);
            }

            if let Some(c) = config {
                Ok(Process::Child(
                    Command::new(&a[0])
                        .args(&a[1..])
                        .stdout(c.stdout.stdio())
                        .stderr(c.stderr.stdio())
                        .stdin(c.stdin.stdio())
                        .spawn()?,
                ))
            } else {
                Ok(Process::Child(Command::new(&a[0]).args(&a[1..]).spawn()?))
            }
        }
        Expr::Binary(left, op, right) => match op {
            Operator::LogicalAnd => match execute(*left, None) {
                Ok(Process::Child(mut child)) => match child.wait() {
                    Ok(status) if status.success() => execute(*right, config),
                    Ok(status) => Ok(Process::ExitStatus(status)),
                    Err(e) => Err(ShellError::from(e)),
                },
                Ok(Process::ExitStatus(status)) => {
                    if status.success() {
                        execute(*right, config)
                    } else {
                        Ok(Process::ExitStatus(status))
                    }
                }
                Err(e) => Err(e),
            },
            Operator::LogicalOr => match execute(*left, None) {
                Ok(Process::Child(mut child)) => match child.wait() {
                    Ok(status) if status.success() => Ok(Process::ExitStatus(status)),
                    _ => execute(*right, config),
                },
                Ok(Process::ExitStatus(status)) => {
                    if status.success() {
                        Ok(Process::ExitStatus(status))
                    } else {
                        execute(*right, config)
                    }
                }
                Err(_) => execute(*right, config),
            },
            Operator::Separator => {
                match execute(*left, None) {
                    Ok(Process::Child(mut child)) => {
                        let _ = child.wait().map_err(|e| eprintln!("rush: {e}"));
                    }
                    Ok(Process::ExitStatus(_)) => {}
                    Err(e) => eprintln!("rush: {e}"),
                }

                Ok(execute(*right, config)?)
            }
            Operator::RedirectOverwrite => {
                let file = if let Expr::Atomic(a) = *right {
                    if a.is_empty() {
                        return Err(ShellError::Unnecassary);
                    }

                    a[0].clone()
                } else {
                    return Err(ShellError::Unnecassary);
                };

                Ok(execute(
                    *left,
                    Some(&Config::new(
                        PathBuf::from(file).stream(FileOption::Overwrite),
                        Stream::Inherit,
                        Stream::Inherit,
                    )),
                )?)
            }
        },
    }
}
