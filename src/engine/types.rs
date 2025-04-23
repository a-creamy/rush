#[derive(Debug)]
pub enum Token {
    Arg(String),
    Pipe,
    And,
    Or,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Token::Arg(arg) => arg,
            Token::Pipe => "|",
            Token::Or => "||",
            Token::And => "&&",
        };
        write!(f, "{}", s)
    }
}

impl Token {
    pub fn precedence(&self) -> u8 {
        match self {
            Token::Arg(_) => 0,
            Token::And | Token::Or => 1,
            Token::Pipe => 2,
        }
    }
}

#[derive(Debug)]
pub enum Cmd {
    Command(Vec<String>),
    BinaryOp(Box<Cmd>, Operator, Box<Cmd>),
}

#[derive(Debug)]
pub enum Operator {
    And,
    Or,
    Pipe,
}
