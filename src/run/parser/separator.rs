use crate::run::{error::ShellError, node::{Ast, Token}};
use super::background;

pub fn parse(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<Ast, ShellError> {
    let mut node = background::parse(tokens)?;

    if let Some(&&Token::Separator) = tokens.peek() {
        tokens.next(); // Consume the `;`
        let rhs = background::parse(tokens)?;
        node = Ast::Separator(Box::new(node), Box::new(rhs));
    }

    Ok(node)
}
