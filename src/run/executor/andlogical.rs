use crate::run::{executor, node::Ast};
use crate::run::ShellError;

pub fn execute(lhs: &Ast, rhs: &Ast) -> Result<(), ShellError> {
    executor::execute(lhs)?;
    executor::execute(rhs)?;
    Ok(())
}
