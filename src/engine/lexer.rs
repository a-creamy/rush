use crate::engine::{error::ShellError, types::Token};
use std::{iter::Peekable, str::Chars};

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, ShellError> {
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(ch) = self.input.next() {
            match ch {
                ' ' | '\t' | '\n' => continue,
                '|' => {
                    if self.input.next_if(|&c| c == '|').is_some() {
                        tokens.push(Token::Or);
                    } else {
                        tokens.push(Token::Pipe);
                    }
                }
                '&' => {
                    if self.input.next_if(|&c| c == '&').is_some() {
                        tokens.push(Token::And);
                    } else {
                        return Err(ShellError::LexerError(format!(
                            "Unknown shell operator '{}'",
                            match self.input.peek() {
                                Some(c) => c,
                                None => {
                                    return Err(ShellError::LexerError(
                                        "Unexpected end of expression".to_string(),
                                    ));
                                }
                            }
                        )));
                    }
                }
                _ => {
                    let mut arg = ch.to_string();

                    while let Some(&c) = self.input.peek() {
                        match c {
                            'a'..='z' | 'A'..='Z' | '.' | '_' | '-' => {
                                self.input.next();
                                arg.push(c);
                            }
                            _ => break,
                        }
                    }

                    tokens.push(Token::Arg(arg));
                }
            }
        }
        Ok(tokens)
    }
}
