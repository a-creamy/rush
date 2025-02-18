use super::{error::ShellError, node::{Ast, Token}};
mod command;
mod background;
mod logical;
mod pipe;
mod redirection;

pub fn parse(tokens: &[Token]) -> Result<Ast, ShellError> {
    background::parse(&mut tokens.iter().peekable())
}
