use std::{
    path::PathBuf,
    process::{Child, Command},
};

use crate::interpreter::{
    error::ShellError,
    parser::{Expr, Operator},
    stream::{FileOption, Stream, StreamChildStdout, StreamFile},
};

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

pub fn execute(expr: Expr, config: Option<&Config>) -> Result<Child, ShellError> {
    match expr {
        Expr::Atomic(a) => {
            if a.is_empty() {
                return Err(ShellError::Unnecassary);
            }

            if let Some(c) = config {
                Ok(Command::new(&a[0])
                    .args(&a[1..])
                    .stdout(c.stdout.stdio())
                    .stderr(c.stderr.stdio())
                    .stdin(c.stdin.stdio())
                    .spawn()?)
            } else {
                Ok(Command::new(&a[0]).args(&a[1..]).spawn()?)
            }
        }
        Expr::Binary(left, op, right) => match op {
            Operator::LogicalAnd => match execute(*left, None) {
                Ok(mut child) => match child.wait() {
                    Ok(status) if status.success() => execute(*right, config),
                    Ok(_) => Err(ShellError::Unnecassary),
                    Err(e) => Err(ShellError::from(e)),
                },
                Err(e) => Err(e),
            },
            Operator::LogicalOr => match execute(*left, None) {
                Ok(mut child) => match child.wait() {
                    Ok(_) => Err(ShellError::Unnecassary),
                    _ => execute(*right, config),
                },
                Err(_) => execute(*right, config),
            },
            Operator::Separator => {
                match execute(*left, None) {
                    Ok(mut child) => {
                        if let Err(e) = child.wait() {
                            eprintln!("rush: {e}");
                        }
                    }
                    Err(e) => eprintln!("rush: {e}"),
                }

                Ok(execute(*right, config)?)
            }
            Operator::RedirectOverwrite(fd) => {
                let file = if let Expr::Atomic(a) = *right {
                    if a.is_empty() {
                        return Err(ShellError::Unnecassary);
                    }

                    a[0].clone()
                } else {
                    return Err(ShellError::Unnecassary);
                };

                match fd {
                    0 => Ok(execute(
                        *left,
                        Some(&Config::new(
                            Stream::Inherit,
                            Stream::Inherit,
                            PathBuf::from(file).stream(FileOption::Overwrite),
                        )),
                    )?),
                    1 => Ok(execute(
                        *left,
                        Some(&Config::new(
                            PathBuf::from(file).stream(FileOption::Overwrite),
                            Stream::Inherit,
                            Stream::Inherit,
                        )),
                    )?),
                    2 => Ok(execute(
                        *left,
                        Some(&Config::new(
                            Stream::Inherit,
                            PathBuf::from(file).stream(FileOption::Overwrite),
                            Stream::Inherit,
                        )),
                    )?),
                    _ => Ok(execute(
                        *left,
                        Some(&Config::new(
                            PathBuf::from(file).stream(FileOption::Overwrite),
                            Stream::Inherit,
                            Stream::Inherit,
                        )),
                    )?),
                }
            }
            Operator::RedirectAppend(fd) => {
                let file = if let Expr::Atomic(a) = *right {
                    if a.is_empty() {
                        return Err(ShellError::Unnecassary);
                    }

                    a[0].clone()
                } else {
                    return Err(ShellError::Unnecassary);
                };

                match fd {
                    0 => Ok(execute(
                        *left,
                        Some(&Config::new(
                            Stream::Inherit,
                            Stream::Inherit,
                            PathBuf::from(file).stream(FileOption::Append),
                        )),
                    )?),
                    1 => Ok(execute(
                        *left,
                        Some(&Config::new(
                            PathBuf::from(file).stream(FileOption::Append),
                            Stream::Inherit,
                            Stream::Inherit,
                        )),
                    )?),
                    2 => Ok(execute(
                        *left,
                        Some(&Config::new(
                            Stream::Inherit,
                            PathBuf::from(file).stream(FileOption::Append),
                            Stream::Inherit,
                        )),
                    )?),
                    _ => Ok(execute(
                        *left,
                        Some(&Config::new(
                            PathBuf::from(file).stream(FileOption::Append),
                            Stream::Inherit,
                            Stream::Inherit,
                        )),
                    )?),
                }
            }
            Operator::Pipe => {
                match execute(
                    *left,
                    Some(&Config::new(
                        Stream::Piped,
                        Stream::Inherit,
                        Stream::Inherit,
                    )),
                ) {
                    Ok(mut child) => {
                        child.wait()?;
                        let c = match config {
                            Some(c) => c,
                            None => &Config::new(Stream::Inherit, Stream::Inherit, Stream::Inherit),
                        };

                        if let Some(mut stdout) = child.stdout {
                            Ok(execute(
                                *right,
                                Some(&Config::new(
                                    c.stdout.clone(),
                                    c.stderr.clone(),
                                    stdout.stream(),
                                )),
                            )?)
                        } else {
                            Ok(execute(*right, config)?)
                        }
                    }
                    Err(e) => Err(e),
                }
            }
        },
    }
}
