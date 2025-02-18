use crate::run::{error::ShellError, node::{Ast, Token}};

pub fn parse(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<Ast, ShellError> {
    let mut args = Vec::new();

    while let Some(&Token::Arg(arg)) = tokens.peek() {
        args.push(arg.clone());
        tokens.next();
    }

    Ok(Ast::Command(args))
}
