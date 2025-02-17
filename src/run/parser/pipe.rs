use crate::run::{error::ShellError, node::{Ast, Token}};
use super::redirection;

pub fn parse(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<Ast, ShellError> {
    let mut node = redirection::parse(tokens)?;

    while let Some(&&Token::Pipe) = tokens.peek() {
        tokens.next(); // Consume `|`
        let rhs = redirection::parse(tokens)?;
        node = Ast::Pipe(Box::new(node), Box::new(rhs));
    }

    Ok(node)
}
