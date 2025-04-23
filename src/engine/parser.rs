use crate::engine::{
    error::ShellError,
    types::{Cmd, Operator, Token},
};
use std::{iter::Peekable, slice::Iter};

pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens: tokens.iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Cmd, ShellError> {
        self.expression(0)
    }

    fn expression(&mut self, precedence: u8) -> Result<Cmd, ShellError> {
        let mut left = self.primary()?;

        while let Some(token) = self.tokens.next() {
            if token.precedence() < precedence {
                break;
            }
            left = self.infix(left, &token)?;
        }

        Ok(left)
    }

    fn primary(&mut self) -> Result<Cmd, ShellError> {
        let mut cmd = Vec::new();

        while let Some(token) = self.tokens.peek() {
            match token {
                Token::Arg(arg) => {
                    self.tokens.next();
                    cmd.push(arg.to_owned());
                }
                _ => break,
            }
        }

        Ok(Cmd::Command(cmd))
    }

    fn infix(&mut self, left: Cmd, token: &Token) -> Result<Cmd, ShellError> {
        match token {
            Token::And => {
                let right = self.expression(token.precedence() + 1)?;
                Ok(Cmd::BinaryOp(
                    Box::new(left),
                    Operator::And,
                    Box::new(right),
                ))
            }
            Token::Or => {
                let right = self.expression(token.precedence() + 1)?;
                Ok(Cmd::BinaryOp(Box::new(left), Operator::Or, Box::new(right)))
            }
            Token::Pipe => {
                let right = self.expression(token.precedence() + 1)?;
                Ok(Cmd::BinaryOp(
                    Box::new(left),
                    Operator::Pipe,
                    Box::new(right),
                ))
            }
            _ => Err(ShellError::ParserError(format!("Unkown shell operator '{}'", token))),
        }
    }
}
