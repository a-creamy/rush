pub enum AST {
    Command(Vec<String>),
    Pipeline {
        lhs: Box<AST>,
        rhs: Box<AST>,
    },
    AndLogical {
        lhs: Box<AST>,
        rhs: Box<AST>,
    },
    Redirection {
        lhs: Box<AST>,
        rhs: Box<AST>,
    }
}

pub enum Token {
    Arg(String),
    And,
    Pipe,
    Redirection,
}
