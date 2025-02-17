use logical::LogicalType;
use redirection::RedirectionType;

use super::{error::ShellError, node::Ast};
mod pipe;
mod command;
mod redirection;
mod logical;
mod background;

pub fn execute(node: &Ast) -> Result<(), ShellError> {
    match node {
        Ast::Command(args) => command::execute(args),
        Ast::Pipe(lhs, rhs) => pipe::execute(lhs, rhs),
        Ast::AndLogical(lhs, rhs) => logical::execute(lhs, rhs, LogicalType::And),
        Ast::OrLogical(lhs, rhs) => logical::execute(lhs, rhs, LogicalType::Or),
        Ast::Background(node) => background::execute(node),
        Ast::InputRedirection(lhs, rhs) => redirection::execute(lhs, rhs, RedirectionType::Input),
        Ast::OverwriteRedirection(lhs, rhs) => redirection::execute(lhs, rhs, RedirectionType::Overwrite),
        Ast::AppendRedirection(lhs, rhs) => redirection::execute(lhs, rhs, RedirectionType::Append),
        Ast::ErrorRedirection(lhs, rhs) => redirection::execute(lhs, rhs, RedirectionType::Error),
    }
}
