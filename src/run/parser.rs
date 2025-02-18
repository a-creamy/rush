use super::{
    error::ShellError,
    node::{Ast, Token},
};
mod background;
mod command;
mod logical;
mod pipe;
mod redirection;
mod separator;

pub fn parse(tokens: &[Token]) -> Result<Ast, ShellError> {
    separator::parse(&mut tokens.iter().peekable())
}
