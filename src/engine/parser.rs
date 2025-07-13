use crate::engine::{error::ShellError, lexer::Token};
use std::{iter::Peekable, slice::Iter};

#[derive(Debug, PartialEq)]
pub enum Operator {
    LogicalAnd,
    LogicalOr,

    Separator,

    RedirectOverwrite,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Atomic(Vec<String>),
    Binary(Box<Expr>, Operator, Box<Expr>),
}

pub fn parse(tokens: &[Token]) -> Result<Expr, ShellError> {
    expression(&mut tokens.iter().peekable(), 0)
}

fn expression(tokens: &mut Peekable<Iter<Token>>, precedence: u8) -> Result<Expr, ShellError> {
    let mut left = prefix(tokens)?;

    while let Some(&token) = tokens.peek() {
        let token_precedence = get_precedence(token);

        if token_precedence < precedence {
            break;
        }

        let token = tokens.next().unwrap();

        left = infix(tokens, left, &token, token_precedence)?;
    }

    Ok(left)
}

fn prefix(tokens: &mut Peekable<Iter<Token>>) -> Result<Expr, ShellError> {
    if let Some(token) = tokens.next() {
        match token {
            Token::Atomic(value) => {
                let mut args = vec![value.clone()];

                while let Some(&&Token::Atomic(_)) = tokens.peek() {
                    if let &Token::Atomic(ref value) = tokens.next().unwrap() {
                        args.push(value.clone());
                    }
                }

                Ok(Expr::Atomic(args))
            }
            _ => Err(ShellError::Parser("Unexpected symbol".into())),
        }
    } else {
        Ok(Expr::Atomic(vec![]))
    }
}

fn infix(
    tokens: &mut Peekable<Iter<Token>>,
    left: Expr,
    token: &Token,
    precedence: u8,
) -> Result<Expr, ShellError> {
    let right = expression(tokens, precedence + 1)?;

    match token {
        Token::LogicalAnd => Ok(Expr::Binary(
            Box::new(left),
            Operator::LogicalAnd,
            Box::new(right),
        )),
        Token::LogicalOr => Ok(Expr::Binary(
            Box::new(left),
            Operator::LogicalOr,
            Box::new(right),
        )),
        Token::Separator => Ok(Expr::Binary(
            Box::new(left),
            Operator::Separator,
            Box::new(right),
        )),
        Token::RedirectOverwrite => Ok(Expr::Binary(
            Box::new(left),
            Operator::RedirectOverwrite,
            Box::new(right),
        )),
        _ => Err(ShellError::Parser("Unexpected infix symbol".into())),
    }
}

fn get_precedence(token: &Token) -> u8 {
    match token {
        Token::Atomic(_) => 0,
        Token::Separator => 1,
        Token::LogicalAnd | Token::LogicalOr => 2,
        Token::RedirectOverwrite => 4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic() {
        let mut tokens = Vec::new();
        tokens.push(Token::Atomic("echo".to_string()));
        tokens.push(Token::Atomic("Hello".to_string()));
        tokens.push(Token::Atomic("World".to_string()));

        let expr = parse(&tokens);

        let mut expected_atomic = Vec::new();
        expected_atomic.push("echo".to_string());
        expected_atomic.push("Hello".to_string());
        expected_atomic.push("World".to_string());

        if let Ok(Expr::Atomic(a)) = expr {
            assert_eq!(a.len(), expected_atomic.len());

            for i in 0..a.len() {
                assert_eq!(a[i], expected_atomic[i]);
            }
        } else {
            panic!("Expected an atomic, didn't get an atomic.")
        }
    }

    #[test]
    fn test_operator() {
        let mut tokens = Vec::new();
        tokens.push(Token::Atomic("echo".to_string()));
        tokens.push(Token::Atomic("Hello".to_string()));
        tokens.push(Token::Atomic("World".to_string()));
        tokens.push(Token::LogicalAnd);
        tokens.push(Token::Atomic("echo".to_string()));
        tokens.push(Token::Atomic("Hi".to_string()));

        let expr = parse(&tokens);

        let mut left_expected_atomic = Vec::new();
        left_expected_atomic.push("echo".to_string());
        left_expected_atomic.push("Hello".to_string());
        left_expected_atomic.push("World".to_string());

        let mut right_expected_atomic = Vec::new();
        right_expected_atomic.push("echo".to_string());
        right_expected_atomic.push("Hi".to_string());

        if let Ok(Expr::Binary(ll, op, rr)) = expr {
            assert_eq!(op, Operator::LogicalAnd);

            if let Expr::Atomic(left) = *ll {
                assert_eq!(left.len(), left_expected_atomic.len());

                for i in 0..left.len() {
                    assert_eq!(left[i], left_expected_atomic[i]);
                }
            } else {
                panic!("Expected the left leaf of binary to be atomic");
            }

            if let Expr::Atomic(right) = *rr {
                assert_eq!(right.len(), right_expected_atomic.len());

                for i in 0..right.len() {
                    assert_eq!(right[i], right_expected_atomic[i]);
                }
            } else {
                panic!("Expected the right leaf of binary to be atomic");
            }
        } else {
            panic!("Expected a binary, didn't get a binary.")
        }
    }

    #[test]
    fn test_empty() {
        let tokens = Vec::new();

        let expr = parse(&tokens);

        if let Ok(Expr::Atomic(a)) = expr {
            assert!(a.is_empty());
        } else {
            panic!("Expected an atomic, didn't get an atomic.")
        }
    }

    #[test]
    fn test_unexpected_op() {
        let mut tokens = Vec::new();
        tokens.push(Token::LogicalAnd);

        let expr = parse(&tokens);

        match expr {
            Ok(_) => panic!("Unexpected result"),
            Err(_) => {
                return;
            }
        }
    }
}
