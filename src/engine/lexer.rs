use crate::engine::error::ShellError;

#[derive(Debug, PartialEq)]
pub enum Token {
    Atomic(String),

    LogicalAnd,
    LogicalOr,

    Separator,

    RedirectOverwrite,
}

pub fn lex(input: String) -> Result<Vec<Token>, ShellError> {
    let mut tokens = Vec::new();

    let mut source = input.chars().peekable();
    while let Some(ch) = source.peek() {
        match ch {
            ' ' | '\t' | '\n' => {
                source.next();
            }
            '&' => {
                source.next();
                if let Some(peek) = source.peek() {
                    if peek.to_owned() == '&' {
                        tokens.push(Token::LogicalAnd);
                        source.next();
                    }
                }
            }
            '|' => {
                source.next();
                if let Some(peek) = source.peek() {
                    if peek.to_owned() == '|' {
                        tokens.push(Token::LogicalOr);
                        source.next();
                    }
                }
            }
            ';' => {
                tokens.push(Token::Separator);
                source.next();
            }
            '>' => {
                tokens.push(Token::RedirectOverwrite);
                source.next();
            }
            'a'..='z' | 'A'..='Z' | '.' | '-' | '_' => {
                let mut atomic = String::new();

                while let Some(c) = source.peek() {
                    if c.is_alphabetic() || c == &'.' || c == &'-' || c == &'_' {
                        atomic.push(c.to_owned());
                        source.next();
                    } else {
                        break;
                    }
                }

                tokens.push(Token::Atomic(atomic));
            }
            _ => {
                return Err(ShellError::Lexer("Unknown symbol".into()));
            }
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic() {
        let tokens = lex("echo Hello World".to_string()).unwrap();
        let mut expected_tokens = Vec::new();
        expected_tokens.push(Token::Atomic("echo".to_string()));
        expected_tokens.push(Token::Atomic("Hello".to_string()));
        expected_tokens.push(Token::Atomic("World".to_string()));

        assert_eq!(tokens.len(), expected_tokens.len());

        for i in 0..tokens.len() {
            assert_eq!(tokens[i], expected_tokens[i]);
        }
    }

    #[test]
    fn test_empty() {
        let tokens = lex("".to_string()).unwrap();
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_operator() {
        let tokens = lex("ls && echo Hi".to_string()).unwrap();
        let mut expected_tokens = Vec::new();
        expected_tokens.push(Token::Atomic("ls".to_string()));
        expected_tokens.push(Token::LogicalAnd);
        expected_tokens.push(Token::Atomic("echo".to_string()));
        expected_tokens.push(Token::Atomic("Hi".to_string()));

        assert_eq!(tokens.len(), expected_tokens.len());

        for i in 0..tokens.len() {
            assert_eq!(tokens[i], expected_tokens[i]);
        }
    }
}
