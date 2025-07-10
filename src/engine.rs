use super::engine::parser::{Expr, Operator};
use std::{
    io::{Error, ErrorKind},
    process::{Child, Command, ExitStatus},
};

pub mod lexer;
pub mod parser;

pub enum Process {
    Child(Child),
    ExitStatus(ExitStatus),
}

pub fn execute(expr: Expr) -> Result<Process, Error> {
    match expr {
        Expr::Atomic(a) => {
            if a.is_empty() {
                return Err(Error::new(ErrorKind::Other, "Empty"));
            }
            Ok(Process::Child(Command::new(&a[0]).args(&a[1..]).spawn()?))
        }
        Expr::Binary(left, op, right) => match op {
            Operator::LogicalAnd => match execute(*left) {
                Ok(Process::Child(mut child)) => match child.wait() {
                    Ok(status) if status.success() => execute(*right),
                    Ok(status) => Ok(Process::ExitStatus(status)),
                    Err(e) => Err(e.into()),
                },
                Ok(Process::ExitStatus(status)) => {
                    if status.success() {
                        execute(*right)
                    } else {
                        Ok(Process::ExitStatus(status))
                    }
                }
                Err(e) => Err(e),
            },
            Operator::LogicalOr => match execute(*left) {
                Ok(Process::Child(mut child)) => match child.wait() {
                    Ok(status) if status.success() => Ok(Process::ExitStatus(status)),
                    _ => execute(*right),
                },
                Ok(Process::ExitStatus(status)) => {
                    if status.success() {
                        Ok(Process::ExitStatus(status))
                    } else {
                        execute(*right)
                    }
                }
                Err(_) => execute(*right),
            },
            Operator::Separator => {
                if let Ok(Process::Child(mut child)) = execute(*left) {
                    let _ = child.wait();
                }
                Ok(execute(*right)?)
            }
        },
    }
}
