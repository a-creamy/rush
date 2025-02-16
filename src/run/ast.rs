use super::{error::ShellError, node::{Ast, Token}};

pub fn parse(tokens: &[Token]) -> Result<Ast, ShellError> {
    let mut tokens = tokens.iter().peekable();
    parse_or_logical(&mut tokens)
}

fn parse_or_logical(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<Ast, ShellError> {
    let mut left = parse_and(tokens)?;
    while let Some(&&Token::OrLogical) = tokens.peek() {
        tokens.next(); // Consume the `||`
        let right = parse_and(tokens)?;
        left = Ast::OrLogical(Box::new(left), Box::new(right));
    }
    Ok(left)
}

fn parse_and(tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Result<Ast, ShellError> {
    let mut left = parse_pipe(tokens)?;
    while let Some(&&Token::AndLogical) = tokens.peek() {
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
        left = Ast::Pipe(Box::new(left), Box::new(right));
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

    let mut command = Ast::Command(args);

    // Handle redirections at the command level
    while let Some(&token) = tokens.peek() {
        match token {
            Token::OverwriteRedirection => {
                tokens.next(); // Consume '>'
                let right = parse_command(tokens)?;
                command = Ast::OverwriteRedirection(Box::new(command), Box::new(right));
            }
            Token::AppendRedirection => {
                tokens.next(); // Consume '>>'
                let right = parse_command(tokens)?;
                command = Ast::AppendRedirection(Box::new(command), Box::new(right));
            }
            Token::ErrorRedirection => {
                tokens.next(); // Consume '2>'
                let right = parse_command(tokens)?;
                command = Ast::ErrorRedirection(Box::new(command), Box::new(right));
            }
            _ => break,
        }
    }

    Ok(command)
}
