use crate::run::{error::ShellError, node::{Ast, Token}};
use super::background;

pub fn parse(tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Result<Ast, ShellError> {
    let mut node = background::parse(tokens)?;
    while let Some(&&Token::Separator) = tokens.peek() {
        tokens.next(); // Consume ';'
        let right = background::parse(tokens)?;
        node = Ast::Separator(Box::new(node), Box::new(right));
    }
    Ok(node)
}
