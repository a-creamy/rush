use super::logical;
use crate::run::{
    error::ShellError,
    node::{Ast, Token},
};

pub fn parse(tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Result<Ast, ShellError> {
    let mut node = logical::parse(tokens)?;

    while let Some(&&Token::Background) = tokens.peek() {
        tokens.next(); // Consume the `&`
        let rhs = logical::parse(tokens)?;
        node = Ast::Background(Box::new(node), Box::new(rhs));
    }

    Ok(node)
}
