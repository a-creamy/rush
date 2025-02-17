use super::{error::ShellError, node::{Ast, Token}};

pub fn parse(tokens: &[Token]) -> Result<Ast, ShellError> {
    let mut tokens = tokens.iter().peekable();
    parse_background(&mut tokens)
}

fn parse_background(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<Ast, ShellError> {
    let mut node = parse_logical(tokens)?;

    if let Some(&&Token::Background) = tokens.peek() {
        tokens.next(); // Consume the `&`
        node = Ast::Background(Box::new(node));
    }

    Ok(node)
}

fn parse_logical(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<Ast, ShellError> {
    let mut node = parse_pipes(tokens)?;

    while let Some(&&Token::OrLogical) = tokens.peek() {
        tokens.next(); // Consume `||`
        let rhs = parse_pipes(tokens)?;
        node = Ast::OrLogical(Box::new(node), Box::new(rhs));
    }

    while let Some(&&Token::AndLogical) = tokens.peek() {
        tokens.next(); // Consume `&&`
        let rhs = parse_pipes(tokens)?;
        node = Ast::AndLogical(Box::new(node), Box::new(rhs));
    }

    Ok(node)
}

fn parse_pipes(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<Ast, ShellError> {
    let mut node = parse_redirections(tokens)?;

    while let Some(&&Token::Pipe) = tokens.peek() {
        tokens.next(); // Consume `|`
        let rhs = parse_redirections(tokens)?;
        node = Ast::Pipe(Box::new(node), Box::new(rhs));
    }

    Ok(node)
}

fn parse_redirections(
    tokens: &mut std::iter::Peekable<std::slice::Iter<Token>>,
) -> Result<Ast, ShellError> {
    let mut node = parse_commands(tokens)?;

    while let Some(&token) = tokens.peek() {
        match token {
            Token::OutputRedirection => {
                tokens.next(); // Consume `&>`
                let rhs = parse_commands(tokens)?;
                node = Ast::OutputRedirection(Box::new(node), Box::new(rhs));
            }
            Token::InputRedirection => {
                tokens.next(); // Consume `<`
                let rhs = parse_commands(tokens)?;
                node = Ast::InputRedirection(Box::new(node), Box::new(rhs));
            }
            Token::OverwriteRedirection => {
                tokens.next(); // Consume `>`
                let rhs = parse_commands(tokens)?;
                node = Ast::OverwriteRedirection(Box::new(node), Box::new(rhs));
            }
            Token::AppendRedirection => {
                tokens.next(); // Consume `>>`
                let rhs = parse_commands(tokens)?;
                node = Ast::AppendRedirection(Box::new(node), Box::new(rhs));
            }
            Token::ErrorRedirection => {
                tokens.next(); // Consume `2>`
                let rhs = parse_commands(tokens)?;
                node = Ast::ErrorRedirection(Box::new(node), Box::new(rhs));
            }
            _ => break, // No more redirections
        }
    }

    Ok(node)
}

fn parse_commands(
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
