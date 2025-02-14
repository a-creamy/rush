pub enum AST {
    Command(Vec<String>),
    Pipeline(Box<AST>, Box<AST>),
    AndLogical(Box<AST>, Box<AST>),
    Redirection(Box<AST>, Box<AST>),
}

pub enum Token {
    Arg(String),
    And,
    Pipe,
    Redirection,
}
