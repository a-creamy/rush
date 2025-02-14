pub enum Operator {
    Pipe,
    And,
    Redirection,
}

pub enum AST {
    Command(Vec<String>),

    #[allow(dead_code)]
    Pipeline {
        operator: Operator,
        lhs: Box<AST>,
        rhs: Box<AST>,
    },

    #[allow(dead_code)]
    AndLogical {
        operator: Operator,
        lhs: Box<AST>,
        rhs: Box<AST>,
    },

    Redirection {
        operator: Operator,
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
