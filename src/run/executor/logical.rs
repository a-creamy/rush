use crate::run::{error::ShellError, node::Ast};
mod or;
mod and;

pub enum LogicalType {
    And,
    Or,
}

pub fn execute(lhs: &Ast, rhs: &Ast, logical_type: LogicalType) -> Result<(), ShellError> {
    match logical_type {
        LogicalType::And => and::execute(lhs, rhs),
        LogicalType::Or => or::execute(lhs, rhs),
    }
}
