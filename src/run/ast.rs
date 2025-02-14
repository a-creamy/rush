use super::error::ShellError;
use super::node::{Token, AST};

pub fn parse(tokens: &[Token]) -> Result<AST, ShellError> {
    let mut tokens = tokens.iter().peekable();
    match parse_append(&mut tokens) {
        Ok(output) => return Ok(output),
        Err(e) => return Err(e),
    }
}

fn parse_append(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>
) -> Result<AST, ShellError> {
    let mut left = parse_redirection(tokens)?;
    while let Some(&&Token::AppendRedirection) = tokens.peek() {
        tokens.next(); // Consume the '>>'
        let right = parse_redirection(tokens)?;
        left = AST::AppendRedirection(Box::new(left), Box::new(right));
    }
    Ok(left)
}

fn parse_redirection(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<AST, ShellError> {
    let mut left = parse_and(tokens)?;
    while let Some(&&Token::OverwriteRedirection) = tokens.peek() {
        tokens.next(); // Consume the '>'
        let right = parse_and(tokens)?;
        left = AST::OverwriteRedirection(Box::new(left), Box::new(right));
    }
    Ok(left)
}

fn parse_and(tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Result<AST, ShellError> {
    let mut left = parse_pipe(tokens)?;
    while let Some(&&Token::And) = tokens.peek() {
        tokens.next(); // Consume the `&&`
        let right = parse_pipe(tokens)?;
        left = AST::AndLogical(Box::new(left), Box::new(right));
    }
    Ok(left)
}

fn parse_pipe(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<AST, ShellError> {
    let mut left = parse_command(tokens)?;
    while let Some(&&Token::Pipe) = tokens.peek() {
        tokens.next(); // Consume the `|`
        let right = parse_command(tokens)?;
        left = AST::Pipeline(Box::new(left), Box::new(right));
    }
    Ok(left)
}

fn parse_command(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<AST, ShellError> {
    let mut args = Vec::new();
    while let Some(&&Token::Arg(ref arg)) = tokens.peek() {
        args.push(arg.clone());
        tokens.next();
    }
    if args.is_empty() {
        return Err(ShellError::ExpectedCommand);
    }
    Ok(AST::Command(args))
}
