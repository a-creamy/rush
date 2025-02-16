use crate::run::{error::ShellError, node::Ast};
mod overwrite;
mod error;
mod append;

pub enum RedirectionType {
    Overwrite,
    Append,
    Error,
}

pub fn execute(lhs: &Ast, rhs: &Ast, redirection_type: RedirectionType) -> Result<(), ShellError> {
    match redirection_type {
        RedirectionType::Overwrite => overwrite::execute(lhs, rhs),
        RedirectionType::Append => append::execute(lhs, rhs),
        RedirectionType::Error => error::execute(lhs, rhs),
    }
}
