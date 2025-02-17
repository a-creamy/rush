use crate::run::{error::ShellError, node::{Ast, Token}};

pub fn parse(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<Ast, ShellError> {
    let mut args = Vec::new();

    while let Some(&Token::Arg(arg)) = tokens.peek() {
        args.push(arg.clone());
        tokens.next();
    }

    if args.is_empty() {
        return Err(ShellError::ExpectedCommand);
    }

    Ok(Ast::Command(args))
}
