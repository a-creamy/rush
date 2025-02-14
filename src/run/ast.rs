use super::error::ShellError;
use super::node::{Operator, Token, AST};

pub fn parse(tokens: &[Token]) -> Result<AST, ShellError> {
    let mut tokens = tokens.iter().peekable();
    match parse_redirection(&mut tokens) {
        Ok(output) => return Ok(output),
        Err(_) => return Err(ShellError::ExpectedCommand),
    }
}

fn parse_redirection(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<AST, ShellError> {
    let mut left = parse_and(tokens)?;
    while let Some(&&Token::Redirection) = tokens.peek() {
        tokens.next(); // Consume the '>'
        let right = parse_and(tokens)?;
        left = AST::Redirection {
            operator: Operator::Redirection,
            lhs: Box::new(left),
            rhs: Box::new(right),
        };
    }
    Ok(left)
}

fn parse_and(tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Result<AST, ShellError> {
    let mut left = parse_pipe(tokens)?;
    while let Some(&&Token::And) = tokens.peek() {
        tokens.next(); // Consume the `&&`
        let right = parse_pipe(tokens)?;
        left = AST::AndLogical {
            operator: Operator::And,
            lhs: Box::new(left),
            rhs: Box::new(right),
        };
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
        left = AST::Pipeline {
            operator: Operator::Pipe,
            lhs: Box::new(left),
            rhs: Box::new(right),
        };
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
