use super::engine::parser::{Expr, Operator};
use std::{
    io::Error,
    process::{Child, Command},
};

pub mod lexer;
pub mod parser;

pub fn execute(expr: Expr) -> Result<Child, Error> {
    match expr {
        Expr::Atomic(a) => Ok(Command::new(&a[0]).args(&a[1..]).spawn()?),
        Expr::Binary(left, op, right) => match op {
            Operator::LogicalAnd => {
                execute(*left).and_then(|mut child| child.wait())?;
                execute(*right)
            }
        },
    }
}
