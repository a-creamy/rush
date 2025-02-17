use crate::run::{error::ShellError, node::{Ast, Token}};
use super::pipe;

pub fn parse(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<Ast, ShellError> {
    let mut node = pipe::parse(tokens)?;

    while let Some(&&Token::OrLogical) = tokens.peek() {
        tokens.next(); // Consume `||`
        let rhs = pipe::parse(tokens)?;
        node = Ast::OrLogical(Box::new(node), Box::new(rhs));
    }

    while let Some(&&Token::AndLogical) = tokens.peek() {
        tokens.next(); // Consume `&&`
        let rhs = pipe::parse(tokens)?;
        node = Ast::AndLogical(Box::new(node), Box::new(rhs));
    }

    Ok(node)
}
