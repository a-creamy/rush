use crate::run::{error::ShellError, node::{Ast, Token}};
use super::command;

pub fn parse(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<Ast, ShellError> {
    let mut node = command::parse(tokens)?;

    while let Some(&token) = tokens.peek() {
        match token {
            Token::OutputRedirection => {
                tokens.next(); // Consume `&>`
                let rhs = command::parse(tokens)?;
                node = Ast::OutputRedirection(Box::new(node), Box::new(rhs));
            }
            Token::InputRedirection => {
                tokens.next(); // Consume `<`
                let rhs = command::parse(tokens)?;
                node = Ast::InputRedirection(Box::new(node), Box::new(rhs));
            }
            Token::OverwriteRedirection => {
                tokens.next(); // Consume `>`
                let rhs = command::parse(tokens)?;
                node = Ast::OverwriteRedirection(Box::new(node), Box::new(rhs));
            }
            Token::AppendRedirection => {
                tokens.next(); // Consume `>>`
                let rhs = command::parse(tokens)?;
                node = Ast::AppendRedirection(Box::new(node), Box::new(rhs));
            }
            Token::ErrorRedirection => {
                tokens.next(); // Consume `2>`
                let rhs = command::parse(tokens)?;
                node = Ast::ErrorRedirection(Box::new(node), Box::new(rhs));
            }
            _ => break, // No more redirections
        }
    }

    Ok(node)
}
