pub enum Operator {
    Pipe,
    And
}

pub enum AST {
    Command(Vec<String>),
    Pipeline {
        operator: Operator,
        lhs: Box<AST>,
        rhs: Box<AST>,
    },
    AndLogical {
        operator: Operator,
        lhs: Box<AST>,
        rhs: Box<AST>,
    }
}

pub enum Token {
    Arg(String),
    And,
    Pipe
}
