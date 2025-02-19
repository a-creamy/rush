use crate::run::{error::ShellError, node::Ast, executor};

pub fn execute(lhs: &Ast, rhs: &Ast) -> Result<(), ShellError> {
    match executor::execute(lhs) {
        Ok(_) => executor::execute(rhs)?,
        Err(e) => {
            eprintln!("{}", e);
            executor::execute(rhs)?
        }
    }
    Ok(())
}
