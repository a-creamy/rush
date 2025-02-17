use crate::run::{error::ShellError, node::Ast};
mod overwrite;
mod error;
mod append;
mod input;
mod output;

pub enum RedirectionType {
    Output,
    Input,
    Overwrite,
    Append,
    Error,
}

pub fn execute(lhs: &Ast, rhs: &Ast, redirection_type: RedirectionType) -> Result<(), ShellError> {
    match redirection_type {
        RedirectionType::Input => input::execute(lhs, rhs),
        RedirectionType::Overwrite => overwrite::execute(lhs, rhs),
        RedirectionType::Append => append::execute(lhs, rhs),
        RedirectionType::Error => error::execute(lhs, rhs),
    }
}
