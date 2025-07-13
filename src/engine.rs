use std::{
    fs::File,
    io::{Error, ErrorKind},
    process::{Child, Command, ExitStatus, Stdio},
};

use super::engine::parser::{Expr, Operator};

pub mod lexer;
pub mod parser;

pub enum Process {
    Child(Child),
    ExitStatus(ExitStatus),
}

pub struct Config {
    pub stdout: Box<dyn Fn() -> Stdio>,
    pub stderr: Box<dyn Fn() -> Stdio>,
    pub stdin: Box<dyn Fn() -> Stdio>,
}

impl Config {
    pub fn new(
        stdout: impl Fn() -> Stdio + 'static,
        stderr: impl Fn() -> Stdio + 'static,
        stdin: impl Fn() -> Stdio + 'static,
    ) -> Self {
        Self {
            stdout: Box::new(stdout),
            stderr: Box::new(stderr),
            stdin: Box::new(stdin),
        }
    }
}

pub fn execute(expr: Expr, config: Option<&Config>) -> Result<Process, Error> {
    match expr {
        Expr::Atomic(a) => {
            if a.is_empty() {
                return Err(Error::new(ErrorKind::Other, "Empty command"));
            }

            if let Some(c) = config {
                Ok(Process::Child(
                    Command::new(&a[0])
                        .args(&a[1..])
                        .stdout((c.stdout)())
                        .stderr((c.stderr)())
                        .stdin((c.stdin)())
                        .spawn()?,
                ))
            } else {
                return Ok(Process::Child(Command::new(&a[0]).args(&a[1..]).spawn()?));
            }
        }
        Expr::Binary(left, op, right) => match op {
            Operator::LogicalAnd => match execute(*left, None) {
                Ok(Process::Child(mut child)) => match child.wait() {
                    Ok(status) if status.success() => execute(*right, config),
                    Ok(status) => Ok(Process::ExitStatus(status)),
                    Err(e) => Err(e),
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
                        let _ = child.wait().map_err(|e| eprintln!("{e}"));
                    }
                    Ok(Process::ExitStatus(_)) => {}
                    Err(e) => eprintln!("{e}"),
                }

                Ok(execute(*right, config)?)
            }
            Operator::RedirectOverwrite => {
                let file = if let Expr::Atomic(a) = *right {
                    if a.is_empty() {
                        return Err(Error::new(ErrorKind::InvalidFilename, "Invalid filename"));
                    }

                    a[0].clone()
                } else {
                    return Err(Error::new(ErrorKind::InvalidFilename, "Invalid filename"));
                };

                Ok(execute(
                    *left,
                    Some(&Config::new(
                        move || {
                            Stdio::from(File::create(file.clone()).expect("Failed to create file"))
                        },
                        || Stdio::inherit(),
                        || Stdio::inherit(),
                    )),
                )?)
            }
        },
    }
}
