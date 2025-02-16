use super::{error::ShellError, node::{Ast, Token}};

pub fn parse(tokens: &[Token]) -> Result<Ast, ShellError> {
    let mut tokens = tokens.iter().peekable();
    parse_error_redirection(&mut tokens)
}

fn parse_error_redirection(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<Ast, ShellError> {
    let mut left = parse_append(tokens)?;
    while let Some(&&Token::ErrorRedirection) = tokens.peek() {
        tokens.next(); // Consume the '2>'
        let right = parse_append(tokens)?;
        left = Ast::ErrorRedirection(Box::new(left), Box::new(right));
    }
    Ok(left)
}

fn parse_append(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<Ast, ShellError> {
    let mut left = parse_redirection(tokens)?;
    while let Some(&&Token::AppendRedirection) = tokens.peek() {
        tokens.next(); // Consume the '>>'
        let right = parse_redirection(tokens)?;
        left = Ast::AppendRedirection(Box::new(left), Box::new(right));
    }
    Ok(left)
}

fn parse_redirection(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<Ast, ShellError> {
    let mut left = parse_and(tokens)?;
    while let Some(&&Token::OverwriteRedirection) = tokens.peek() {
        tokens.next(); // Consume the '>'
        let right = parse_and(tokens)?;
        left = Ast::OverwriteRedirection(Box::new(left), Box::new(right));
    }
    Ok(left)
}

fn parse_and(tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Result<Ast, ShellError> {
    let mut left = parse_pipe(tokens)?;
    while let Some(&&Token::And) = tokens.peek() {
        tokens.next(); // Consume the `&&`
        let right = parse_pipe(tokens)?;
        left = Ast::AndLogical(Box::new(left), Box::new(right));
    }
    Ok(left)
}

fn parse_pipe(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<Ast, ShellError> {
    let mut left = parse_command(tokens)?;
    while let Some(&&Token::Pipe) = tokens.peek() {
        tokens.next(); // Consume the `|`
        let right = parse_command(tokens)?;
        left = Ast::Pipeline(Box::new(left), Box::new(right));
    }
    Ok(left)
}

fn parse_command(
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
