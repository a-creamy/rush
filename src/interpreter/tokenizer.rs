use super::node::{RedirectType, Token};
use std::str::Chars;

pub struct Tokenizer<'a> {
    input: Chars<'a>,
    current_char: Option<char>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut tokenizer = Tokenizer {
            input: input.chars(),
            current_char: None,
        };
        tokenizer.advance();
        tokenizer
    }

    pub fn advance(&mut self) {
        self.current_char = self.input.next();
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(c) = self.current_char {
            match c {
                '|' => {
                    self.advance();
                    if self.current_char == Some('|') {
                        tokens.push(Token::OrLogical);
                    } else {
                        tokens.push(Token::Pipe);
                    }
                }
                '&' => {
                    self.advance();
                    if self.current_char == Some('&') {
                        tokens.push(Token::AndLogical);
                    } else if self.current_char == Some('>') {
                        tokens.push(Token::Redirect(RedirectType::Anything));
                    } else {
                        tokens.push(Token::Background);
                    }
                    self.advance();
                }
                '>' => {
                    self.advance();
                    if self.current_char == Some('>') {
                        tokens.push(Token::Redirect(RedirectType::Append))
                    } else {
                        tokens.push(Token::Redirect(RedirectType::Output));
                    }
                    self.advance();
                }
                '<' => {
                    self.advance();
                    tokens.push(Token::Redirect(RedirectType::Input));
                }
                ' ' | '\t' | '\n' => self.advance(),
                _ => {
                    let mut arg = String::new();

                    while let Some(_) = self.current_char {
                        if self.current_char.unwrap().is_whitespace()
                            || "|&<".contains(self.current_char.unwrap())
                        {
                            break;
                        }

                        // For "2>"
                        if self.current_char.unwrap() == '2' {
                            self.advance();
                            if self.current_char.unwrap() == '>' {
                                tokens.push(Token::Redirect(RedirectType::Error));
                                self.advance();
                                continue;
                            }
                            arg.push(self.current_char.unwrap())
                        }

                        arg.push(self.current_char.unwrap());
                        self.advance();
                    }

                    if !arg.is_empty() {
                        tokens.push(Token::Arg(arg));
                    }
                }
            }
        }

        tokens
    }
}
