use super::node::{Ast, Token};
use std::iter::Peekable;
use std::slice::Iter;

pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser {
            tokens: tokens.iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Ast, String> {
        self.parse_expression(0)
    }

    fn parse_expression(&mut self, precedence: u8) -> Result<Ast, String> {
        let mut left = self.parse_primary()?;

        while let Some(&token) = self.tokens.peek() {
            let token_precedence = Parser::<'a>::get_precedence(token);

            if token_precedence < precedence {
                break;
            }

            let token = self.tokens.next().unwrap();

            left = self.parse_infix(left, &token, token_precedence)?;
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Ast, String> {
        if let Some(token) = self.tokens.next() {
            match token {
                Token::Arg(value) => {
                    let mut args = vec![value.clone()];

                    while let Some(&&Token::Arg(_)) = self.tokens.peek() {
                        if let &Token::Arg(ref value) = self.tokens.next().unwrap() {
                            args.push(value.clone());
                        }
                    }

                    Ok(Ast::Command(args))
                }
                _ => Err(format!("Unexpected token: {:?}", token)),
            }
        } else {
            Err("Unexpected end of tokens".to_string())
        }
    }

    fn parse_infix(&mut self, left: Ast, token: &Token, precedence: u8) -> Result<Ast, String> {
        match token {
            Token::Pipe => {
                let right = self.parse_expression(precedence + 1)?;
                Ok(Ast::Pipe(Box::new(left), Box::new(right)))
            }
            Token::Background => {
                if self.tokens.peek().is_some() {
                    let right = self.parse_expression(precedence + 1)?;
                    Ok(Ast::Background(Box::new(left), Box::new(right)))
                } else {
                    Ok(Ast::Background(
                        Box::new(left),
                        Box::new(Ast::Command(vec![])),
                    ))
                }
            }
            Token::AndLogical => {
                let right = self.parse_expression(precedence + 1)?;
                Ok(Ast::AndLogical(Box::new(left), Box::new(right)))
            }
            Token::OrLogical => {
                let right = self.parse_expression(precedence + 1)?;
                Ok(Ast::OrLogical(Box::new(left), Box::new(right)))
            }
            Token::Redirect(redirect_type) => {
                let right = self.parse_expression(precedence + 1)?;
                Ok(Ast::Redirect(
                    Box::new(left),
                    Box::new(right),
                    redirect_type.clone(),
                ))
            }
            _ => Err(format!("Unexpected infix token: {:?}", token)),
        }
    }

    pub fn get_precedence(token: &Token) -> u8 {
        match token {
            Token::Background => 10,
            Token::AndLogical | Token::OrLogical => 20,
            Token::Pipe => 30,
            Token::Redirect(_) => 40,
            Token::Arg(_) => 50,
        }
    }
}
