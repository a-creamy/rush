use redirection::RedirectionType;
use super::{error::ShellError, node::Ast};
mod pipeline;
mod command;
mod andlogical;
mod redirection;

pub fn execute(node: &Ast) -> Result<(), ShellError> {
    match node {
        Ast::Command(args) => command::execute(args),
        Ast::Pipeline(lhs, rhs) => pipeline::execute(lhs, rhs),
        Ast::AndLogical(lhs, rhs) => andlogical::execute(lhs, rhs),
        Ast::OverwriteRedirection(lhs, rhs) => redirection::execute(lhs, rhs, RedirectionType::Overwrite),
        Ast::AppendRedirection(lhs, rhs) => redirection::execute(lhs, rhs, RedirectionType::Append),
        Ast::ErrorRedirection(lhs, rhs) => redirection::execute(lhs, rhs, RedirectionType::Error),
    }
}
