mod background;
mod command;
mod logical;
mod pipe;
mod redirection;

use super::{error::ShellError, node::Ast};
use logical::LogicalType;
use redirection::RedirectionType;

pub fn execute(node: &Ast) -> Result<(), ShellError> {
    match node {
        Ast::Command(args) => command::execute(args),
        Ast::Pipe(lhs, rhs) => pipe::execute(lhs, rhs),
        Ast::Background(lhs, rhs) => background::execute(lhs, rhs),
        Ast::AndLogical(lhs, rhs) => logical::execute(lhs, rhs, LogicalType::And),
        Ast::OrLogical(lhs, rhs) => logical::execute(lhs, rhs, LogicalType::Or),
        Ast::InputRedirection(lhs, rhs) => redirection::execute(lhs, rhs, RedirectionType::Input),
        Ast::OutputRedirection(lhs, rhs) => redirection::execute(lhs, rhs, RedirectionType::Output),
        Ast::OverwriteRedirection(lhs, rhs) => redirection::execute(lhs, rhs, RedirectionType::Overwrite),
        Ast::AppendRedirection(lhs, rhs) => redirection::execute(lhs, rhs, RedirectionType::Append),
        Ast::ErrorRedirection(lhs, rhs) => redirection::execute(lhs, rhs, RedirectionType::Error),
    }
}
