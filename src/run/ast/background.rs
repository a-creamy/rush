use crate::run::{error::ShellError, node::{Ast, Token}};
use super::logical;

pub fn parse(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<Ast, ShellError> {
    let mut node = logical::parse(tokens)?;

    if let Some(&&Token::Background) = tokens.peek() {
        tokens.next(); // Consume the `&`
        node = Ast::Background(Box::new(node));
    }

    Ok(node)
}
