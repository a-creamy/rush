use crate::run::{executor, node::Ast, ShellError};

pub fn execute(lhs: &Ast, rhs: &Ast) -> Result<(), ShellError> {
    executor::execute(lhs)?;
    executor::execute(rhs)?;
    Ok(())
}
